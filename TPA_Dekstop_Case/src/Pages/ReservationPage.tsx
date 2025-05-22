import NavigationBar from "../Navbarnya/Navbar";
import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import "react-datepicker/dist/react-datepicker.css";
import ChooseBranch from "./ChooseBranch";
import { useLocation, useNavigate } from "react-router";

interface TableData {
  id: string;
  capacity: number;
  is_available: boolean;
  position: { x: number; y: number };
}

interface WaitingListEntry {
  id: number;
  user_id: number;
  customer_name: string;
  people_count: number;
  requested_time: string;
  created_at: string;
}

interface ReservationEntry {
  id: string;
  user_id: number;
  customer_name: string;
  people_count: number;
  time_slot: string;
  created_at: string;
  time_limit: number;
}

function ReservationPage() {
  const [mode, setMode] = useState<"auto" | "manual">("auto");
  const [timeLimit, setTimeLimit] = useState<number>(60);
  const [selectedTables, setSelectedTables] = useState<string[]>([]);
  const [selectedDate, setSelectedDate] = useState<Date>(new Date());
  const [peopleCount, setPeopleCount] = useState<number>(1);
  const [customerName, setCustomerName] = useState<string>("");
  const [tables, setTables] = useState<TableData[]>([]);
  const [loading, setLoading] = useState(true);
  const [waitingList, setWaitingList] = useState<WaitingListEntry[]>([]);
  const [reservations, setReservations] = useState<ReservationEntry[]>([]);
  const [error, setError] = useState<string | null>(null);

  const location = useLocation();
  const navigate = useNavigate();
  const queryParams = new URLSearchParams(location.search);
  const branch_name = queryParams.get("branch");
  const address = queryParams.get("address");
  const user_id = queryParams.get("user_id");

  const formatDateForInput = (date: Date): string => {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");
    const hours = String(date.getHours()).padStart(2, "0");
    const minutes = String(date.getMinutes()).padStart(2, "0");
    return `${year}-${month}-${day}T${hours}:${minutes}`;
  };

  const handleTableClick = (tableId: string) => {
    setSelectedTables((prev) =>
      prev.includes(tableId)
        ? prev.filter((id) => id !== tableId)
        : [...prev, tableId]
    );
  };

  const fetchBranchData = async () => {
    try {
      const [waitingListData, reservationsData] = await Promise.all([
        invoke<WaitingListEntry[]>("get_branch_waiting_list", { address }),
        invoke<ReservationEntry[]>("get_branch_reservations", { address }),
      ]);

      setWaitingList(waitingListData);
      setReservations(reservationsData);
    } catch (error) {
      console.error("Failed to fetch branch data:", error);
    }
  };

  useEffect(() => {
    if (address) {
      fetchBranchData();
    }
  }, [address]);

  // const handleSubmit = async (e: React.FormEvent) => {
  //   e.preventDefault();
  //   try {
  //     const reservationData = {
  //       user_id: 1,
  //       branch_id: 1,
  //       customer_name: customerName,
  //       people_count: peopleCount,
  //       time_slot: selectedDate.toISOString(),
  //       table_ids: mode === "manual" ? selectedTables : null,
  //     };

  //     console.log("Submitting reservation:", reservationData);
  //   } catch (error) {
  //     console.error("Reservation failed:", error);
  //     alert("Failed to make reservation");
  //   }
  // };

  useEffect(() => {
    if (address) {
      fetchAvailableTables();
    }
  }, [address, selectedDate]);

  const fetchAvailableTables = async () => {
    try {
      const tablesData = await invoke<TableData[]>("get_available_tables", {
        address,
        timeSlot: selectedDate.toISOString(),
      });
      console.log("Raw response from backend:", tablesData);
      console.log(
        "Fetched tables with availability:",
        tablesData.map((t) => ({ id: t.id, isAvailable: t.is_available }))
      );
      setTables(tablesData);
      setLoading(false);
    } catch (error) {
      console.error("Failed to fetch tables:", error);
      setError("Failed to load available tables");
      setLoading(false);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!user_id || !address || !branch_name) {
      alert("Missing required parameters");
      return;
    }

    const naiveDateTime = selectedDate.toISOString().replace(/\.\d+Z$/, "");

    try {
      const reservationData = {
        user_id: parseInt(user_id),
        address,
        customer_name: customerName,
        people_count: peopleCount,
        time_slot: naiveDateTime,
        time_limit: timeLimit,
        table_ids:
          mode === "manual" ? selectedTables.map((id) => id.toString()) : null,
      };

      console.log("Data being sent:", {
        ...reservationData,
        time_slot: reservationData.time_slot,
        table_ids: reservationData.table_ids,
      });

      const result = await invoke("create_reservation", {
        request: reservationData,
      });

      alert(
        "Reservation created successfully! Please order within " + timeLimit
      );
      navigate("/");
    } catch (error) {
      console.error("Reservation failed:", error);
      alert("Failed to create reservation. Please try again.");
    }
  };

  if (!address || !branch_name || !user_id) {
    return (
      <div className="min-h-screen bg-slate-100 p-8 flex items-center justify-center">
        <div className="text-red-500">Missing required parameters</div>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-slate-100 p-8 flex items-center justify-center">
        <div>Loading tables...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-slate-100 p-8">
      <NavigationBar />
      <div className="max-w-4xl mx-auto bg-white rounded-lg shadow-md p-6">
        <h1 className="text-2xl font-bold mb-6">
          Table Reservation at {branch_name}
        </h1>

        <div className="flex gap-4 mb-6">
          <button
            onClick={() => setMode("auto")}
            className={`px-4 py-2 rounded transition-colors ${
              mode === "auto"
                ? "bg-blue-500 text-white"
                : "bg-gray-200 hover:bg-gray-300"
            }`}
          >
            Auto Assignment
          </button>
          <button
            onClick={() => setMode("manual")}
            className={`px-4 py-2 rounded transition-colors ${
              mode === "manual"
                ? "bg-blue-500 text-white"
                : "bg-gray-200 hover:bg-gray-300"
            }`}
          >
            Manual Selection
          </button>
        </div>

        <form onSubmit={handleSubmit} className="space-y-6">
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium mb-1">
                Customer Name
              </label>
              <input
                type="text"
                value={customerName}
                onChange={(e) => setCustomerName(e.target.value)}
                className="w-full p-2 border rounded"
                required
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">
                Number of People
              </label>
              <input
                type="number"
                min="1"
                value={peopleCount}
                onChange={(e) => setPeopleCount(parseInt(e.target.value))}
                className="w-full p-2 border rounded"
                required
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">
                Date & Time
              </label>
              <input
                type="datetime-local"
                value={formatDateForInput(selectedDate)}
                onChange={(e) => setSelectedDate(new Date(e.target.value))}
                className="w-full p-2 border rounded"
                min={formatDateForInput(new Date())}
                required
              />
            </div>
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">
              Time Limit (minutes)
            </label>
            <select
              value={timeLimit}
              onChange={(e) => setTimeLimit(Number(e.target.value))}
              className="w-full p-2 border rounded"
              required
            >
              <option value={60}>1 hour</option>
              <option value={90}>1.5 hours</option>
              <option value={120}>2 hours</option>
              <option value={180}>3 hours</option>
            </select>
            <p className="text-sm text-gray-500 mt-1">
              Please complete your order within the selected time limit
            </p>
          </div>

          {mode === "manual" && (
            <div className="mt-6">
              <h2 className="text-lg font-semibold mb-4">Select Tables</h2>
              <div className="grid grid-cols-4 gap-4">
                {tables.map((table, index) => (
                  <div
                    key={table.id}
                    onClick={() => {
                      console.log("Table : ", table.is_available);
                      if (table.is_available ?? true) {
                        handleTableClick(table.id);
                      }
                    }}
                    className={`
                      p-4 rounded-lg text-center cursor-pointer transition-all
                      ${
                        !(table.is_available ?? true)
                          ? "bg-gray-300 cursor-not-allowed"
                          : selectedTables.includes(table.id)
                          ? "bg-blue-500 text-white"
                          : "bg-green-100 hover:bg-green-200"
                      }
                    `}
                  >
                    <div className="font-bold">Table {index + 1}</div>
                    <div className="text-sm">
                      {table.is_available ?? true ? "Available" : "Occupied"}
                    </div>
                    <div className="text-sm">Seats: {table.capacity}</div>
                  </div>
                ))}
              </div>
            </div>
          )}

          <button
            type="submit"
            className="w-full bg-blue-500 text-white py-2 rounded hover:bg-blue-600 
                             transition-colors"
          >
            Submit Reservation
          </button>
        </form>
        <div className="mt-8 space-y-8">
          <div>
            <h2 className="text-xl font-semibold mb-4">Current Reservations</h2>
            <div className="bg-white rounded-lg shadow overflow-hidden">
              <table className="min-w-full">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                      Customer
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                      People
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                      Time
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                      Time Limit
                    </th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-200">
                  {reservations.map((reservation) => (
                    <tr key={reservation.id}>
                      <td className="px-6 py-4">{reservation.customer_name}</td>
                      <td className="px-6 py-4">{reservation.people_count}</td>
                      <td className="px-6 py-4">
                        {new Date(reservation.time_slot).toLocaleString()}
                      </td>
                      <td className="px-6 py-4">
                        {reservation.time_limit} minutes
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

          <div>
            <h2 className="text-xl font-semibold mb-4">Waiting List</h2>
            <div className="bg-white rounded-lg shadow overflow-hidden">
              <table className="min-w-full">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                      Customer
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                      People
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                      Requested Time
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                      Waiting Since
                    </th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-200">
                  {waitingList.map((entry) => (
                    <tr key={entry.id}>
                      <td className="px-6 py-4">{entry.customer_name}</td>
                      <td className="px-6 py-4">{entry.people_count}</td>
                      <td className="px-6 py-4">
                        {new Date(entry.requested_time).toLocaleString()}
                      </td>
                      <td className="px-6 py-4">
                        {new Date(entry.created_at).toLocaleString()}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default ReservationPage;
