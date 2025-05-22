import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import NavigationBar from "../Navbarnya/Navbar";
import { useLocation } from "react-router";

export default function JobApplicationPage() {
  const [role, setRole] = useState("");
  const [branch, setBranch] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const location = useLocation();
  const queryParams = new URLSearchParams(location.search);
  const user_id = parseInt(queryParams.get("user_id") || "0");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      await invoke("apply_for_job", {
        userId: user_id,
        role,
        branchAddress: branch,
      });

      alert("Application submitted successfully!");
      setRole("");
      setBranch("");
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-100">
      <NavigationBar />
      <div className="max-w-2xl mx-auto pt-10 px-4">
        <h1 className="text-3xl font-bold mb-8">Job Application</h1>

        {error && (
          <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
          <div className="mb-4">
            <label className="block text-gray-700 text-sm font-bold mb-2">
              Select Role
            </label>
            <select
              value={role}
              onChange={(e) => setRole(e.target.value)}
              className="shadow border rounded w-full py-2 px-3 text-gray-700"
              required
            >
              <option value="">Select a role</option>
              <option value="Branch Manager">Branch Manager</option>
              <option value="Branch Marketing Staff">Branch Marketing Staff</option>
              <option value="Branch Operational Staff">Branch Operational Staff</option>
              <option value="Cashier">Cashier</option>
              <option value="Chef">Chef</option>
            </select>
          </div>

          <div className="mb-6">
            <label className="block text-gray-700 text-sm font-bold mb-2">
              Select Branch
            </label>
            <input
              type="text"
              value={branch}
              onChange={(e) => setBranch(e.target.value)}
              className="shadow border rounded w-full py-2 px-3 text-gray-700"
              required
              placeholder="Enter branch address"
            />
          </div>

          <div className="flex items-center justify-end">
            <button
              type="submit"
              disabled={loading}
              className={`bg-blue-500 text-white font-bold py-2 px-4 rounded
                ${loading ? 'opacity-50 cursor-not-allowed' : 'hover:bg-blue-700'}`}
            >
              {loading ? 'Submitting...' : 'Submit Application'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}