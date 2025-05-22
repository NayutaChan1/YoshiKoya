import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router";
import NavigationBar from "../Navbarnya/Navbar";

interface Branch {
  branch_name: string;
  branch_address: string;
  opening_time: string;
  closing_time: string;
}

export default function BranchList() {
  const [branches, setBranches] = useState<Branch[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();

  useEffect(() => {
    fetchBranches();
  }, []);

  const fetchBranches = async () => {
    try {
      const result = await invoke<Branch[]>("get_all_branches");
      setBranches(result);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const handleBranchClick = (branch: Branch) => {
    navigate(`/branchdetail?address=${branch.branch_address}`);
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-100 flex items-center justify-center">
        <div className="text-xl font-semibold">Loading branches...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-gray-100 p-8">
        <div className="bg-red-100 text-red-700 p-4 rounded-lg">
          Error: {error}
        </div>
      </div>
    );
  }

  return (
    <div className="bg-slate-300 min-h-screen p-0 m-0">
      <NavigationBar></NavigationBar>
      <div className="max-w-6xl mx-auto">
        <h1 className="text-3xl font-bold mb-6">Our Branches</h1>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {branches.map((branch) => (
            <div
              key={branch.branch_address}
              onClick={() => handleBranchClick(branch)}
              className="bg-white rounded-lg shadow-md p-6 cursor-pointer 
                       transform transition-transform hover:scale-105"
            >
              <h2 className="text-xl font-semibold mb-2">
                {branch.branch_name}
              </h2>
              <p className="text-gray-600 mb-4">{branch.branch_address}</p>
              <div className="text-sm text-gray-500">
                <p>Opening: {branch.opening_time}</p>
                <p>Closing: {branch.closing_time}</p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
