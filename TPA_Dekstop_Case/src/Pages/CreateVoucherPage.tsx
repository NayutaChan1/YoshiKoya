import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSearchParams } from "react-router";
import NavigationBar from "../Navbarnya/Navbar";

interface Menu {
    name: string;
    price: number;
    menu_type: string;
    image_bytes: string;
    address: string;
  }

export default function CreateVoucherPage() {
  const [searchParams] = useSearchParams();
  const branch = searchParams.get("branch");
  const [menus, setMenus] = useState<Menu[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  // Form states
  const [selectedMenu, setSelectedMenu] = useState("");
  const [code, setCode] = useState("");
  const [discountPercent, setDiscountPercent] = useState("");
  const [startDate, setStartDate] = useState("");
  const [expiryDate, setExpiryDate] = useState("");

  useEffect(() => {
    if (branch) {
      fetchBranchMenus();
    }
  }, [branch]);

  const fetchBranchMenus = async () => {
    try {
      const result = await invoke<Menu[]>("get_branch_menus", {
        branchAddress: branch
      });
      console.log("Menus:", result);
      setMenus(result);
    } catch (err) {
      setError(String(err));
    }
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toISOString().slice(0, 19).replace("T", " ");
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      await invoke("create_menu_voucher", {
        menuName: selectedMenu,
        code,
        discountPercent: parseFloat(discountPercent),
        startDate: formatDate(startDate),
        expiryDate: formatDate(expiryDate),
      });

      setSelectedMenu("");
      setCode("");
      setDiscountPercent("");
      setStartDate("");
      setExpiryDate("");

      alert("Voucher created successfully!");
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  if (!branch) {
    return <div>No branch selected</div>;
  }

  return (
    <div className="min-h-screen bg-gray-100">
      <NavigationBar />
      <div className="max-w-4xl mx-auto pt-8 px-4">
        <h1 className="text-3xl font-bold mb-8">Create Menu Voucher</h1>
        
        {error && (
          <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
          <div className="mb-4">
            <label className="block text-gray-700 text-sm font-bold mb-2">
              Select Menu
            </label>
            <select
              value={selectedMenu}
              onChange={(e) => setSelectedMenu(e.target.value)}
              className="shadow border rounded w-full py-2 px-3 text-gray-700"
              required
            >
              <option value="">Select a menu item</option>
              {menus.map((menu) => (
                <option key={menu.name} value={menu.name}>
                  {menu.name} - {menu.menu_type}
                </option>
              ))}
            </select>
          </div>

          <div className="mb-4">
            <label className="block text-gray-700 text-sm font-bold mb-2">
              Voucher Code
            </label>
            <input
              type="text"
              value={code}
              onChange={(e) => setCode(e.target.value)}
              className="shadow border rounded w-full py-2 px-3 text-gray-700"
              required
              placeholder="Enter voucher code"
            />
          </div>

          <div className="mb-4">
            <label className="block text-gray-700 text-sm font-bold mb-2">
              Discount Percentage
            </label>
            <input
              type="number"
              min="0"
              max="100"
              value={discountPercent}
              onChange={(e) => setDiscountPercent(e.target.value)}
              className="shadow border rounded w-full py-2 px-3 text-gray-700"
              required
              placeholder="Enter discount percentage"
            />
          </div>

          <div className="mb-4">
            <label className="block text-gray-700 text-sm font-bold mb-2">
              Start Date
            </label>
            <input
              type="datetime-local"
              value={startDate}
              onChange={(e) => setStartDate(e.target.value)}
              className="shadow border rounded w-full py-2 px-3 text-gray-700"
              required
            />
          </div>

          <div className="mb-6">
            <label className="block text-gray-700 text-sm font-bold mb-2">
              Expiry Date
            </label>
            <input
              type="datetime-local"
              value={expiryDate}
              onChange={(e) => setExpiryDate(e.target.value)}
              className="shadow border rounded w-full py-2 px-3 text-gray-700"
              required
            />
          </div>

          <div className="flex items-center justify-end">
            <button
              type="submit"
              disabled={loading}
              className={`bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded
                ${loading ? 'opacity-50 cursor-not-allowed' : ''}`}
            >
              {loading ? 'Creating...' : 'Create Voucher'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}