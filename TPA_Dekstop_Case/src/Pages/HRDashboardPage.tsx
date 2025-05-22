import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import NavigationBar from "../Navbarnya/Navbar";

interface JobApplication {
  id: number;
  user_id: number;
  role: string;
  branch_address: string;
  status: string;
  applied_at: string;
}

export default function HRDashboardPage() {
  const [applications, setApplications] = useState<JobApplication[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchApplications();
  }, []);

  const fetchApplications = async () => {
    try {
      const result = await invoke<JobApplication[]>("get_pending_applications");
      setApplications(result);
    } catch (err) {
      alert("Failed to fetch applications: " + err);
      setError("Failed to fetch applications: " + err);
    } finally {
      setLoading(false);
    }
  };

  const handleUpdateStatus = async (userId: number, status: string) => {
    try {
      await invoke("update_job_status", {
        userId,
        status,
      });
      
      fetchApplications();
    } catch (err) {
      alert("Failed to update status: " + err);
      setError("Failed to update status: " + err);
    }
  };

  if (loading) {
    return <div>Loading...</div>;
  }

  return (
    <div className="min-h-screen bg-gray-100">
      <NavigationBar />
      <div className="max-w-6xl mx-auto pt-10 px-4">
        <h1 className="text-3xl font-bold mb-8">HR Dashboard - Job Applications</h1>

        {error && (
          <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            {error}
          </div>
        )}

        <div className="bg-white shadow-md rounded-lg overflow-hidden">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  User ID
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Role
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Branch
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Applied Date
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {applications.map((app) => (
                <tr key={app.id}>
                  <td className="px-6 py-4 whitespace-nowrap">{app.user_id}</td>
                  <td className="px-6 py-4 whitespace-nowrap">{app.role}</td>
                  <td className="px-6 py-4 whitespace-nowrap">{app.branch_address}</td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    {new Date(app.applied_at).toLocaleDateString()}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <button
                      onClick={() => handleUpdateStatus(app.user_id, "ACCEPTED")}
                      className="bg-green-500 text-white px-3 py-1 rounded mr-2 hover:bg-green-600"
                    >
                      Accept
                    </button>
                    <button
                      onClick={() => handleUpdateStatus(app.user_id, "REJECTED")}
                      className="bg-red-500 text-white px-3 py-1 rounded hover:bg-red-600"
                    >
                      Reject
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}