import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useParams, useNavigate } from "react-router";
import NavigationBar from "../Navbarnya/Navbar";
import { useSearchParams } from 'react-router';


interface Branch {
  branch_name: string;
  branch_address: string;
  opening_time: string;
  closing_time: string;
}

export default function BranchDetail() {
  const [searchParams] = useSearchParams();
  const address = searchParams.get("address");
  const navigate = useNavigate();
  const [branch, setBranch] = useState<Branch | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isClosing, setIsClosing] = useState(false);

  useEffect(() => {
    if (address) {
      fetchBranchDetails(address);
    }
  }, [address]);

  const fetchBranchDetails = async (branchAddress: string) => {
    try {
      const result = await invoke<Branch>("get_branch", {
        address,
      });
      setBranch(result);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const handleCloseBranch = async () => {
    if (!branch || !confirm("Are you sure you want to close this branch?")) {
      return;
    }

    setIsClosing(true);
    try {
      await invoke("close_branch", {
        address,
      });
      navigate("/mainpage");
    } catch (err) {
      setError(String(err));
    } finally {
      setIsClosing(false);
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-100 flex items-center justify-center">
        <div className="text-xl font-semibold">Loading branch details...</div>
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

  if (!branch) {
    return (
      <div className="min-h-screen bg-gray-100 p-8">
        <div className="text-xl font-semibold">Branch not found</div>
      </div>
    );
  }

  return (
    <div className="bg-slate-300 min-h-screen p-0 m-0">
      <NavigationBar></NavigationBar>
      <div className="max-w-4xl mx-auto bg-white rounded-lg shadow-md p-8">
        <div className="flex justify-between items-start mb-6">
          <h1 className="text-3xl font-bold">{branch.branch_name}</h1>
          <button
            onClick={() => navigate("/branches")}
            className="text-gray-600 hover:text-gray-800"
          >
            Back to Branches
          </button>
        </div>

        <div className="space-y-4">
          <div>
            <h2 className="text-lg font-semibold">Address</h2>
            <p className="text-gray-600">{branch.branch_address}</p>
          </div>

          <div>
            <h2 className="text-lg font-semibold">Operating Hours</h2>
            <p className="text-gray-600">
              {branch.opening_time} - {branch.closing_time}
            </p>
          </div>

          <div className="pt-6">
            <button
              onClick={handleCloseBranch}
              disabled={isClosing}
              className={`px-4 py-2 rounded-lg text-white font-medium
                ${
                  isClosing
                    ? "bg-gray-400 cursor-not-allowed"
                    : "bg-red-600 hover:bg-red-700"
                }`}
            >
              {isClosing ? "Closing Branch..." : "Close Branch"}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
