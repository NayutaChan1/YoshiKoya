import { useEffect, useState } from "react";
import "../App.css";
import NavigationBar from "../Navbarnya/Navbar";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router";

// type MenuWithBranch = {
//     menu_name: string;
//     branch_name: string;
//     branch_address: string;
//     image_bytes: string | null;
//     price: number;
//     menu_type: string;
// };

type MenuWithImage = {
  name: string;
  price: number;
  menu_type: string;
  image_bytes: string | null;
  address: string | null;
};

const MainPage = () => {
  const [menus, setMenus] = useState<MenuWithImage[]>([]);
  const [user, setUserId] = useState<number | null>(null);
  const [searchTerm, setSearchTerm] = useState("");
  const [selectedType, setSelectedType] = useState("");
  const [sortOrder, setSortOrder] = useState<"asc" | "desc">("asc");
  const [currentPage, setCurrentPage] = useState(1);
  const itemsPerPage = 8;
  const navigate = useNavigate();

//   type UserData = {
//     user_name: string;
//     user_id: number;
//   };
  // useEffect(() => {
  //     const fetchMenus = async () => {
  //         try {
  //             const response: MenuWithImage[] = await invoke("fetch_and_check_images");
  //             // console.log("Menunya:", response);
  //             setMenus(response);
  //         } catch (error) {
  //             console.error("Error fetching menus:", error);
  //         }
  //     };

  //     fetchMenus();
  // }, []);

  useEffect(() => {
    const fetchMenus = async () => {
      try {
        const response: MenuWithImage[] = await invoke("get_all_menus");
        setMenus(response);
      } catch (error) {
        console.error("Error fetching menus with branches:", error);
      }
    };

    fetchMenus();
  }, []);

  const handleMenuClick = (menu: MenuWithImage) => {
    console.log("Menu clicked:", menu);
    navigate(`/purchase?menu=${menu.name}&user_id=${user}`);
  };

  const filteredMenus = menus.filter(menu => {
    const matchesType = !selectedType || menu.menu_type === selectedType;
    const matchesSearch = menu.name.toLowerCase().includes(searchTerm.toLowerCase());
    return matchesType && matchesSearch;
  }).sort((a, b) => 
    sortOrder === "asc" ? a.price - b.price : b.price - a.price
  );

  const handleSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchTerm(e.target.value);
    setCurrentPage(1);
  };

  const totalPages = Math.ceil(filteredMenus.length / itemsPerPage);
  const currentItems = filteredMenus.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage
  );

  const menuTypes = Array.from(new Set(menus.map((menu) => menu.menu_type)));

  return (
    <div className="bg-slate-300 min-h-screen p-0 m-0">
      <NavigationBar />

      <div className="p-4 bg-white shadow-md m-4 rounded-lg">
        <div className="flex flex-wrap gap-4 items-center">
          <select
            value={selectedType}
            onChange={(e) => setSelectedType(e.target.value)}
            className="p-2 border rounded-md"
          >
            <option value="">All Types</option>
            {menuTypes.map((type) => (
              <option key={type} value={type}>
                {type}
              </option>
            ))}
          </select>

          <input
            type="text"
            placeholder="Search menu..."
            value={searchTerm}
            onChange={handleSearch}
            className="p-2 border rounded-md flex-grow"
          />

          <button
            onClick={() =>
              setSortOrder((prev) => (prev === "asc" ? "desc" : "asc"))
            }
            className="p-2 bg-blue-500 text-white rounded-md hover:bg-blue-600"
          >
            Price: {sortOrder === "asc" ? "↑" : "↓"}
          </button>
        </div>
      </div>

      {filteredMenus.length === 0 && (
        <div className="text-center p-4 text-gray-600">
          No menus found matching your search criteria
        </div>
      )}

      <div className="p-4 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
        {currentItems.map((menu, index) => (
          <div
            key={index}
            className="bg-white shadow-md rounded-lg overflow-hidden hover:shadow-lg transition-shadow"
            onClick={() =>
              handleMenuClick({
                name: menu.name,
                address: menu.address,
                image_bytes: menu.image_bytes,
                price: menu.price,
                menu_type: menu.menu_type,
              })
            }
          >
            {menu.image_bytes ? (
              <img
                src={`${menu.image_bytes}`}
                alt={menu.name}
                className="w-full h-48 object-cover"
              />
            ) : (
              <div className="w-full h-48 bg-gray-200 flex items-center justify-center">
                <span className="text-gray-500">No Image</span>
              </div>
            )}

            <div className="p-4">
              <h3 className="text-lg font-bold">{menu.name}</h3>
              <p className="text-gray-700">Price: ${menu.price.toFixed(2)}</p>
              <p className="text-gray-500">Type: {menu.menu_type}</p>
            </div>
          </div>
        ))}
      </div>

      {totalPages > 1 && (
        <div className="flex justify-center p-4 gap-2">
          <button
            onClick={() => setCurrentPage((prev) => Math.max(prev - 1, 1))}
            disabled={currentPage === 1}
            className={`px-4 py-2 rounded-md ${
              currentPage === 1
                ? "bg-gray-300"
                : "bg-blue-500 text-white hover:bg-blue-600"
            }`}
          >
            Previous
          </button>

          <span className="px-4 py-2">
            Page {currentPage} of {totalPages}
          </span>

          <button
            onClick={() =>
              setCurrentPage((prev) => Math.min(prev + 1, totalPages))
            }
            disabled={currentPage === totalPages}
            className={`px-4 py-2 rounded-md ${
              currentPage === totalPages
                ? "bg-gray-300"
                : "bg-blue-500 text-white hover:bg-blue-600"
            }`}
          >
            Next
          </button>
        </div>
      )}
    </div>
  );
};

export default MainPage;
