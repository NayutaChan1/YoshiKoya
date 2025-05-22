import { useEffect, useState } from "react";
import "../App.css";
import { useLocation } from "react-router";
import NavigationBar from "../Navbarnya/Navbar";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router";

interface OperationHours {
  opening_time: string;
  closing_time: string;
  is_open: boolean;
}

interface MenuWithImage {
  name: string;
  price: number;
  menu_type: string;
  image_bytes: string | null;
  address: string | null;
}

interface Branch {
  branch_name: string,
  branch_address: string,
  opening_time: string,
  closing_time: string,
  is_open?: boolean,
}

function PurchasePage() {
  const location = useLocation();
  const queryParams = new URLSearchParams(location.search);
  const menuName = queryParams.get("menu");
  const user_id = queryParams.get("user_id");

  const [menuDetail, setMenuDetail] = useState<MenuWithImage | null>(null);
  const [operationHours, setOperationHours] = useState<Branch | null>(null);
  const [isOpen, setIsOpen] = useState<boolean | null>(null);
  const [quantity, setQuantity] = useState(1);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const fetchMenuDetails = async () => {
      if (!menuName) return;
      
      try {
        setIsLoading(true);
        const details = await invoke<MenuWithImage>('get_menu_details', { menuName });
        setMenuDetail(details);
        
        if (details.address) {
          console.log("Fetching hours for address:", details.address);
        
          const hours = await invoke<Branch>('get_branch_hours', {
            address: details.address,
          });
          setOperationHours(hours);
          
          const isOpen = await invoke<boolean>('calculate_is_open', { 
            address: details.address 
          });
          setIsOpen(isOpen);
        }
      } catch (error) {
        console.error("Detailed error:", {
          message: error instanceof Error ? error.message : String(error),
          error: error
      });
      } finally {
        setIsLoading(false);
      }
    };

    fetchMenuDetails();
  }, [menuName]);

  const handleAddToCart = async () => {
    if (!menuDetail || !isOpen) {
      alert(`Sorry, this branch is currently closed.\nOperation Hours: ${operationHours?.opening_time} - ${operationHours?.closing_time}`);
      return;
    }

    try {
      const request = {
        user_id: user_id,
        menu_name: menuDetail.name,
        branch_address: menuDetail.address || "",
        price: menuDetail.price,
        menu_type: menuDetail.menu_type,
        quantity: quantity
    };
      console.log("Adding to cart:", request);
      await invoke("add_to_cart", { request });
      alert("Successfully added to cart!");
    } catch (error) {
      console.error("Error adding to cart:", error);
      alert("Failed to add item to cart. Please try again.");
    }
  };

  if (isLoading) {
    return (
      <div className="min-h-screen bg-slate-300 flex items-center justify-center">
        <div className="text-xl font-semibold">Loading...</div>
      </div>
    );
  }

  return (
    <div className="bg-slate-300 min-h-screen">
      <NavigationBar />
      <div className="container mx-auto px-4 py-8">
        <div className="bg-white rounded-lg shadow-md p-6">
          <div className="mb-6">
            {menuDetail?.image_bytes ? (
              <img
                src={menuDetail.image_bytes}
                alt={menuDetail.name}
                className="w-full h-64 object-cover rounded-lg"
              />
            ) : (
              <div className="w-full h-64 bg-gray-200 rounded-lg flex items-center justify-center">
                <span className="text-gray-500">No Image Available</span>
              </div>
            )}
          </div>

          <div className="mb-6">
            <h1 className="text-2xl font-bold mb-4">{menuDetail?.name}</h1>
            <div className="space-y-2">
              <p><span className="font-medium">Type:</span> {menuDetail?.menu_type}</p>
              <p><span className="font-medium">Price:</span> ${menuDetail?.price.toFixed(2)}</p>
              {operationHours && (
                <>
                  <p><span className="font-medium">Operation Hours:</span> {operationHours.opening_time} - {operationHours.closing_time}</p>
                  <p>
                    <span className="font-medium">Status: </span>
                    <span className={isOpen ? "text-green-600" : "text-red-600 font-bold"}>
                      {isOpen ? "Open" : "Closed"}
                    </span>
                  </p>
                </>
              )}
            </div>
          </div>

          <div className="flex items-center gap-4 mb-6">
            <span className="font-medium">Quantity:</span>
            <div className="flex items-center">
              <button
                onClick={() => setQuantity(q => Math.max(1, q - 1))}
                className="bg-red-500 text-white px-4 py-2 rounded-l hover:bg-red-600 transition-colors"
              >
                -
              </button>
              <span className="px-6 py-2 bg-gray-100 border-y">{quantity}</span>
              <button
                onClick={() => setQuantity(q => q + 1)}
                className="bg-green-500 text-white px-4 py-2 rounded-r hover:bg-green-600 transition-colors"
              >
                +
              </button>
            </div>
          </div>

          <div className="space-y-4">
            <p className="text-xl font-bold">
              Total: ${((menuDetail?.price || 0) * quantity).toFixed(2)}
            </p>
            <button
              onClick={handleAddToCart}
              disabled={!isOpen || !menuDetail}
              className={`w-full py-3 rounded-lg transition-colors ${
                isOpen && menuDetail
                  ? "bg-blue-500 hover:bg-blue-600 text-white"
                  : "bg-gray-400 text-gray-200 cursor-not-allowed"
              }`}
            >
              {!menuDetail ? "Loading..." : isOpen ? "Add to Cart" : "Branch is Closed"}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default PurchasePage;
