import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../App.css";
import NavigationBar from "../Navbarnya/Navbar";
import { useLocation, useNavigate } from "react-router";

interface Branch {
  branch_name: string;
  branch_address: string;
  opening_time: string;
  closing_time: string;
  is_open?: boolean;
}

function ChooseBranch() {
  const location = useLocation();

  const queryParams = new URLSearchParams(location.search);
  const user_id = queryParams.get("user_id");

  const [branches, setBranches] = useState<Branch[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();

  useEffect(() => {
    fetchBranches();
  }, []);

  const fetchBranches = async () => {
    try {
      const branchesData = await invoke<Branch[]>("get_all_branches");
      setBranches(branchesData);

      const branchesWithStatus = await Promise.all(
        branchesData.map(async (branch) => {
          const isOpen = await invoke<boolean>("calculate_is_open", {
            address: branch.branch_address,
          });
          return { ...branch, is_open: isOpen };
        })
      );
      setBranches(branchesWithStatus);
    } catch (error) {
      console.error("Error fetching branches:", error);
      setError("Failed to fetch branches. Please try again later.");
    } finally {
      setIsLoading(false);
    }
  };

  const handleBranchClick = (branch: Branch) => {
    navigate(`/reservation?branch=${branch.branch_name}&address=${branch.branch_address}&user_id=${user_id}`);
  }

  if(isLoading){
    return (
        <div className="min-h-screen bg-slate-300 flex items-center justify-center">
            <h1 className="text-xl">Loading...</h1>
        </div>
    );
  }

  if(error){
    return (
        <div className="bg-slate-300 min-h-screen p-0 m-0 flex items-center justify-center">
            <h1 className="text-xl text-red-600">{error}</h1>
        </div>
    );
  }

  return (
    <div className="bg-slate-300 min-h-screen p-0 m-0">
        <NavigationBar />
        <div className="container mx-auto px-4 py-8">
            <h1 className="text-4xl font-bold mb-8 text-center">Choose a Branch</h1>
            
            <div className="flex flex-col space-y-4 max-w-md mx-auto">
                {branches.map((branch, index) => (
                    <button
                        key={index}
                        onClick={() => handleBranchClick(branch)}
                        className="bg-white hover:bg-blue-50 text-left p-6 rounded-lg shadow-md 
                                 transition-all duration-200 transform hover:scale-105"
                    >
                        <h2 className="text-xl font-semibold text-blue-600 mb-2">
                            {branch.branch_name}
                        </h2>
                        <p className="text-gray-600 mb-2">{branch.branch_address}</p>
                        <p className="text-sm text-gray-500">
                            Operating Hours: {branch.opening_time} - {branch.closing_time}
                        </p>
                        <div className={`mt-2 text-sm ${branch.is_open ? 'text-green-500' : 'text-red-500'}`}>
                            {branch.is_open ? 'Open Now' : 'Closed'}
                        </div>
                    </button>
                ))}
            </div>
        </div>
    </div>
);
}
export default ChooseBranch;
