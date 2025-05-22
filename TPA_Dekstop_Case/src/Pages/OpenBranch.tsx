import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import NavigationBar from "../Navbarnya/Navbar";

export default function OpenBranchPage() {
  const [branchName, setBranchName] = useState("");
  const [branchAddress, setBranchAddress] = useState("");
  const [openTime, setOpenTime] = useState("");
  const [closeTime, setCloseTime] = useState("");
  const [message, setMessage] = useState("");
  const [loading, setLoading] = useState(false);

  const handleCreateBranch = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    try {
      const result = await invoke('create_branch', {
        branchName,
        branchAddress,
        openingTime: openTime,
        closingTime: closeTime,
      });
      setMessage("Branch created successfully!");
      setBranchName("");
      setBranchAddress("");
      setOpenTime("");
      setCloseTime("");
    } catch (err) {
      setMessage(`Failed to create branch: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-100 py-8">
      <NavigationBar />
      
      <div className="max-w-xl mx-auto bg-white rounded-lg shadow-md p-6 m-10">
        <h1 className='text-2xl font-bold mb-6'>Create New Branch</h1>
  
        <form onSubmit={handleCreateBranch} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Branch Name
            </label>
            <input
              type="text"
              required
              placeholder="Enter branch name"
              value={branchName}
              onChange={(e) => setBranchName(e.target.value)}
              className="w-full border p-2 rounded focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
  
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Branch Address</label>
            <input
              type="text"
              placeholder="Enter branch address"
              className="w-full border p-2 rounded focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              value={branchAddress}
              onChange={(e) => setBranchAddress(e.target.value)}
              required
            />
          </div>
  
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Opening Time</label>
              <input
                type="time"
                className="w-full border p-2 rounded focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                value={openTime}
                onChange={(e) => setOpenTime(e.target.value)}
                required
              />
            </div>
  
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Closing Time
              </label>
              <input
                type="time"
                value={closeTime}
                required
                onChange={(e) => setCloseTime(e.target.value)}
                className="w-full border p-2 rounded focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              />
            </div>
          </div>
  
          <button
            type="submit"
            disabled={loading}
            className={`w-full py-2 px-4 rounded text-white font-medium ${
              loading 
                ? 'bg-gray-400 cursor-not-allowed' 
                : 'bg-blue-600 hover:bg-blue-700'
            }`}
          >
            {loading ? 'Creating...' : 'Create Branch'}
          </button>
  
          {message && (
            <div className={`p-3 rounded ${
              message.includes('Failed') 
                ? 'bg-red-100 text-red-700' 
                : 'bg-green-100 text-green-700'
            }`}>
              {message}
            </div>
          )}
        </form>
      </div>
    </div>
  );
}