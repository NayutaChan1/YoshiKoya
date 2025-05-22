import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import NavigationBar from "../Navbarnya/Navbar";

interface Employee {
    user_id: number;
    job: string;
    employee_code: string;
    address: string;
    level: string;
}

export default function EmployeeManagement() {
  const [employees, setEmployees] = useState<Employee[]>([]);
  const [searchTerm, setSearchTerm] = useState("");
  const [selectedRole, setSelectedRole] = useState("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const roles = [
    "Branch Manager",
    "Branch Marketing Staff",
    "Branch HR",
    "Branch Operation Staff",
    "Cashier",
    "Chef",
    "Waiter",
    "Delivery Personnel",
    "Supplier",
    "Corporate HR",
    "General Manager"
  ];

  useEffect(() => {
    fetchEmployees();
  }, []);

  const fetchEmployees = async () => {
    try {
      const result = await invoke<Employee[]>("get_all_employees");
      setEmployees(result);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const handleReassignEmployee = async (userId: number, newJob: string) => {
    try {
      await invoke("reassign_employee", {
        userId, newJob: newJob,
      });
      fetchEmployees();
    } catch (err) {
      setError(String(err));
    }
  };

  const validateEmployeeLevel = (employee: Employee) => {
    const levelMapping: { [key: string]: string } = {
      "CEO": "CEO",
      "Cashier": "Restaurant",
      "Chef": "Restaurant",
      "Waiter": "Restaurant",
      "Delivery Personnel": "Restaurant",
      "Supplier": "Restaurant",
      "Branch Manager": "Branch",
      "Branch Marketing Staff": "Branch",
      "Branch HR": "Branch",
      "Branch Operational Staff": "Branch",
      "Corporate HR": "Corporate",
      "General Manager": "Corporate"
    };

    return levelMapping[employee.job] === employee.level;
  };

  const filteredEmployees = employees.filter(employee => {
    const matchesSearch = employee.employee_code;
    const matchesRole = !selectedRole || employee.job === selectedRole;
    return matchesSearch && matchesRole;
  });

  if (loading) return <div>Loading...</div>;

  return (
    <div className="min-h-screen bg-gray-100">
      <NavigationBar />
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <h1 className="text-3xl font-bold mb-8">Employee Management</h1>

        <div className="mb-6 flex gap-4">
          <input
            type="text"
            placeholder="Search by name..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="p-2 border rounded"
          />

          <select
            value={selectedRole}
            onChange={(e) => setSelectedRole(e.target.value)}
            className="p-2 border rounded"
          >
            <option value="">All Roles</option>
            {roles.map(role => (
              <option key={role} value={role}>{role}</option>
            ))}
          </select>
        </div>

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
                  Employee Code
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Name
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Role
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Level
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Branch
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {filteredEmployees.map((employee) => (
                <tr key={employee.user_id}>
                  <td className="px-6 py-4 whitespace-nowrap">
                    {employee.employee_code}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">{employee.employee_code}</td>
                  <td className="px-6 py-4 whitespace-nowrap">{employee.job}</td>
                  <td className="px-6 py-4 whitespace-nowrap">{employee.level}</td>
                  <td className="px-6 py-4 whitespace-nowrap">{employee.address}</td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full 
                      ${validateEmployeeLevel(employee) 
                        ? 'bg-green-100 text-green-800' 
                        : 'bg-red-100 text-red-800'}`}
                    >
                      {validateEmployeeLevel(employee) ? 'Valid' : 'Invalid Level'}
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <select
                      onChange={(e) => handleReassignEmployee(employee.user_id, e.target.value)}
                      className="p-1 border rounded text-sm"
                    >
                      <option value="">Reassign Role</option>
                      {roles.map(role => (
                        <option key={role} value={role}>{role}</option>
                      ))}
                    </select>
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