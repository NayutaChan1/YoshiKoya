import NavigationBar from "../Navbarnya/Navbar";
import { invoke } from "@tauri-apps/api/core";
import React from "react";
import { useEffect, useState } from "react";
import { useLocation } from "react-router";

interface OperationHours {
  opening_time: string;
  closing_time: string;
  is_open: boolean;
}

function SetOperationPage() {

  type Branch = {
    branch_name: string,
    branch_address: string,
    opening_time: string,
    closing_time: string,
    is_open?: boolean,
  }

  const location = useLocation();
  const queryParams = new URLSearchParams(location.search);

  const branchName = queryParams.get("branchname");
  const branchAddress = queryParams.get("branchaddress") ?? undefined

  const [operationHours, setOperationHours] = useState<OperationHours | null>(null);
  const [newOpeningTime, setNewOpeningTime] = useState("");
  const [newClosingTime, setNewClosingTime] = useState("");
  const [updateMessage, setUpdateMessage] = useState("");
  const [isOpen, setIsOpen] = useState<boolean | null>(null);

  useEffect(() => {
    if (branchName) {
      getBranch(branchAddress);
    }
  }, [branchName]);

  const getBranch = async (address?: string) => {
    try {
      const response = await invoke<Branch>("get_branch", { address });
      // setOperationHours(response);
      setNewOpeningTime(response.opening_time);
      setNewClosingTime(response.closing_time);
      const isOpen =await invoke<boolean>("calculate_is_open", { address });
      setIsOpen(isOpen);
      // console.log(isOpen);
    } catch (error) {
      console.error("Error fetching branch data:", error);
    }
  };


  // const fetchOperationHours = async () => {
  //   try {
  //     const hours = await invoke<OperationHours>("get_branch_hours", {
  //       branchName
  //     });
  //     setOperationHours(hours);
  //     setNewOpeningTime(hours.opening_time);
  //     setNewClosingTime(hours.closing_time);
  //   } catch (error) {
  //     console.error("Error fetching operation hours:", error);
  //   }
  // };

  const handleUpdateHours = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await invoke("update_branch_hours", {
        request: {
          branch_address: branchAddress,
          opening_time: newOpeningTime.trim(),
          closing_time: newClosingTime.trim(),
        }
      });
      setUpdateMessage("Operation hours updated successfully!");
      // fetchOperationHours();
      getBranch(branchAddress);
    } catch (error) {
      setUpdateMessage(`Error updating operation hours: ${error}`);
    }
  };

  return (
    <div className="min-h-screen bg-gray-100">
      <NavigationBar />
      <div className="container mx-auto px-4 py-8">
        <div className="bg-white rounded-lg shadow-md p-6">
          <h1 className="text-2xl font-bold mb-6">Branch Operation Hours</h1>
          
          <div className="mb-6">
            <h2 className="text-xl font-semibold mb-2">Branch Details</h2>
            <div className="bg-gray-50 p-4 rounded">
              <p className="mb-2"><span className="font-medium">Branch Name:</span> {branchName}</p>
              <p><span className="font-medium">Address:</span> {branchAddress}</p>
            </div>
          </div>

          <div className="mb-6">
            <h2 className="text-xl font-semibold mb-2">Current Operation Hours</h2>
            <div className="bg-gray-50 p-4 rounded">
              <p className="mb-2">
                <span className="font-medium">Opening Time:</span> {newOpeningTime}
              </p>
              <p>
                <span className="font-medium">Closing Time:</span> {newClosingTime}
              </p>
              <p className="mt-2">
                <span className="font-medium">Status:</span>{' '}
                <span className={isOpen ? "text-green-600" : "text-red-600"}>
                  {isOpen ? "Currently Open" : "Currently Closed"}
                </span>
              </p>
            </div>
          </div>

          <div>
            <h2 className="text-xl font-semibold mb-4">Update Operation Hours</h2>
            <form onSubmit={handleUpdateHours} className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-1">Opening Time</label>
                <input
                  type="time"
                  value={newOpeningTime}
                  onChange={(e) => setNewOpeningTime(e.target.value)}
                  className="w-full border rounded-md p-2"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">Closing Time</label>
                <input
                  type="time"
                  value={newClosingTime}
                  onChange={(e) => setNewClosingTime(e.target.value)}
                  className="w-full border rounded-md p-2"
                  required
                />
              </div>
              <button
                type="submit"
                className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700"
              >
                Update Hours
              </button>
            </form>

            {updateMessage && (
              <div className={`mt-4 p-3 rounded ${
                updateMessage.includes("Error") ? "bg-red-100 text-red-700" : "bg-green-100 text-green-700"
              }`}>
                {updateMessage}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default SetOperationPage;