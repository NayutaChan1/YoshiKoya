import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router";

function NavigationBar() {
  const navigate = useNavigate();
  const [user, setUserId] = useState<number | null>(null);
  const [greeting, setGreeting] = useState("GakLogin");
  const [dropdownOpen, setDropdownOpen] = useState(false);
  const [jobStatus, setJobStatus] = useState<string | null>(null);
  const [employeeCode, setEmployeeCode] = useState<string | null>(null);
  const [address, setAddress] = useState<string | null>(null);
  const [branchName, setBranchName] = useState<string | null>(null);
  const [branchAddress, setBranchAddress] = useState<string | null>(null);

  type UserData = {
    user_name: string;
    user_id: number;
  };

  type Employee = {
    user_id: number;
    job: string;
    employee_code: string;
    address: string;
    level: string;
  };

  type Branch = {
    branch_name: string;
    branch_address: string;
    opening_time: string;
    closing_time: string;
    is_open?: boolean;
  };

  useEffect(() => {
    const token = localStorage.getItem("sessionToken");

    if (!token) {
      console.warn("Kasian Ga Ketemu");
      return;
    }

    const checkUserSession = async () => {
      try {
        const response = await invoke<UserData>("check_session_handler", {
          token,
        });

        if (response) {
          setGreeting(response.user_name);
          setUserId(response.user_id);
        }
      } catch (error) {
        console.error("Error checking session:", error);
      }
    };

    checkUserSession();
  }, []);

  const getBranchInfo = async (address: string) => {
    try {
      // const branchInfo = await invoke<{ branch_name: string; branch_address: string } | null>(
      //   "get_branch_info",
      //   { employeeCode }
      // );

      const branchInfo2 = await invoke<Branch>("get_branch", { address });

      if (branchInfo2) {
        console.log("Branch Info:", branchInfo2);
        setBranchName(branchInfo2.branch_name);
        setBranchAddress(branchInfo2.branch_address);
      } else {
        console.log("There is no tempat kerja:", employeeCode);
      }
    } catch (error) {
      console.error("Error fetching:", error);
    }
  };

  useEffect(() => {
    const token = localStorage.getItem("sessionToken");

    if (!token) {
      console.warn("Kasian Ga Ketemu");
      return;
    }

    const checkUserSession = async () => {
      try {
        const response = await invoke<UserData>("check_session_handler", {
          token,
        });

        if (response) {
          setGreeting(response.user_name);
          setUserId(response.user_id);

          console.log("User ID:", response.user_id);
          // const job = await getjob(response.user_id);
          // const empCode = await getempcode(response.user_id);
          const employee = await getEmployee(response.user_id);
          console.log("Employee data:", employee);

          setJobStatus(employee?.job.toString() || null);
          setEmployeeCode(employee?.employee_code.toString() || null);
          setAddress(employee?.address.toString() || null);
        }
      } catch (error) {
        console.error("Error checking session:", error);
      }
    };

    checkUserSession();
  }, []);

  const handleLogout = () => {
    localStorage.removeItem("sessionToken");
    navigate("/");
  };

  // async function checkUserSession(token: string) {
  //     try {
  //         const response = await invoke("check_session", { session: { token } });
  //         console.log("Session status:", response);
  //     } catch (error) {
  //         console.error("Error checking session:", error);
  //     }
  //   }

  async function getEmployee(userId: number): Promise<Employee | null> {
    try {
      const response = await invoke<Employee>("get_employee", { userId });
      console.log("Employee data:", response);
      return response;
    } catch (error) {
      console.log("Error fetching employee data:", error);
      return null;
    }
  }

  //ini diganti
  // async function getjob(userId: number): Promise<string | null> {
  //   try {
  //     const response = await invoke<string>("get_user_job", { userId });
  //     console.log("Job status:", response);
  //     return response;
  //   } catch (error) {
  //     console.error("Error fetching job status:", error);
  //     return null;
  //   }
  // }

  // //ini juga diganti nanti
  // async function getempcode(userId: number): Promise<string | null> {
  //   try {
  //     const response = await invoke<string>("get_employee_code", { userId });
  //     console.log("Employee Code:", response);
  //     return response;
  //   } catch (error) {
  //     console.error("Error fetching employee code:", error);
  //     return null;
  //   }
  // }

  // getempcode(user!);
  // getjob(user!);
  getBranchInfo(address!);
  // const userToken = localStorage.getItem("user_token");

  const navbarOptions: Record<
    string,
    { label: string; onClick: () => void }[]
  > = {
    "Branch Manager": [
      {
        label: "Home",
        onClick: () => navigate("/mainpage"),
      },
      {
        label: "Set Operations Hours",
        onClick: () =>
          navigate(
            `/setoperation?branchname=${branchName}&branchaddress=${branchAddress}`
          ),
      },
    ],
    "Branch Operational Staff": [
      {
        label: "Home",
        onClick: () => navigate("/mainpage"),
      },
      {
        label: "Set Operation Hours",
        onClick: () =>
          navigate(
            `/setoperation?branchname=${branchName}&branchaddress=${branchAddress}`
          ),
      },
    ],
    "Branch Marketing Staff": [
      {
        label: "Home",
        onClick: () => navigate("/mainpage"),
      },
      {
        label: "Create Voucher",
        onClick: () => navigate(`/create-voucher?branch=${branchAddress}`),
      },
    ],
    "General Manager": [
      {
        label: "Home",
        onClick: () => navigate("/mainpage"),
      },
      {
        label: "View Branch",
        onClick: () => navigate(`/viewallbranch`),
      },
      {
        label: "Open New Branch",
        onClick: () => navigate("/openbranch"),
      },
    ],
    "Corporate HR": [
      {
        label: "Home",
        onClick: () => navigate("/mainpage"),
      },
      {
        label: "Job Applications",
        onClick: () => navigate("/hr-dashboard"),
      },
      {
        label: "Employee Management",
        onClick: () => navigate("/employee-management"),
      },
    ],
    Cashier: [
      {
        label: "Process Orders",
        onClick: () => navigate("/processorders"),
      },
      {
        label: "View Transactions",
        onClick: () => navigate("/transactions"),
      },
    ],
    Chef: [
      {
        label: "View Recipes",
        onClick: () => navigate("/recipes"),
      },
      {
        label: "Manage Inventory",
        onClick: () => navigate("/inventory"),
      },
    ],
    default: [
      {
        label: "Home",
        onClick: () => navigate("/mainpage"),
      },
      {
        label: "Order",
        onClick: () => navigate("/order"),
      },
      {
        label: "Cart",
        onClick: () => navigate(`/cart?user_id=${user}`),
      },
      {
        label: "Reservation",
        onClick: () => navigate(`/choosebranch?user_id=${user}`),
      },
      {
        label: "Apply Job",
        onClick: () => navigate(`/apply-job?user_id=${user}`),
      },
    ],
  };

  const options =
    navbarOptions[jobStatus || "default"] || navbarOptions["default"];

  return (
    <div>
      <div className="bg-gray-900 flex justify-between items-center p-5 w-full">
        <div className="flex items-center space-x-8">
          <img
            src="src/img/DALL·E 2025-03-28 02.27.41 - A modern and elegant logo for 'YoshiKoya' restaurant. The design incorporates Japanese elements such as a stylized ramen bowl with chopsticks, subtle .webp"
            alt="Logo YoshiKoya"
            className="h-full w-auto max-h-16"
          />
          <div className="flex space-x-6 text-white font-medium">
            {options &&
              options.map((option, index) => (
                <span
                  key={index}
                  className="cursor-pointer hover:text-gray-300"
                  onClick={option.onClick}
                >
                  {option.label}
                </span>
              ))}
          </div>
        </div>

        <div className="relative ml-auto">
          {employeeCode ? (
            <div
              className="flex items-center space-x-2 cursor-pointer"
              onClick={() => setDropdownOpen(!dropdownOpen)}
            >
              <span className="text-white font-medium">
                {jobStatus || "Employee"} - {employeeCode}
              </span>
            </div>
          ) : (
            <div
              className="flex items-center space-x-2 cursor-pointer"
              onClick={() => setDropdownOpen(!dropdownOpen)}
            >
              <img
                src="src\img\ci-2GT4EerpG87LszAv0WhYGPtn9MLHI2kH1637144512.jpg"
                alt="Profile"
                className="h-10 w-10 rounded-full"
              />
            </div>
          )}
          {dropdownOpen && (
            <div className="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg z-10">
              <ul className="py-1">
                <li className="px-4 py-2 text-gray-700 hover:bg-gray-100 cursor-pointer">
                  Profile Details
                </li>
                <li
                  className="px-4 py-2 text-gray-700 hover:bg-gray-100 cursor-pointer"
                  onClick={handleLogout}
                >
                  Logout
                </li>
              </ul>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default NavigationBar;
