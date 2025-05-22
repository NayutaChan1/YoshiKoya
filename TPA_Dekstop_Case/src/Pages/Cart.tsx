import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../App.css";
import NavigationBar from "../Navbarnya/Navbar";
import { useLocation } from "react-router";

interface Voucher {
  id: number;
  menu_name: string;
  code: string;
  discount_percent: number;
  expiry_date: string;
}

function CartPage() {
  const [cartItems, setCartItems] = useState<any[]>([]);
  const [voucher, setVoucher] = useState<Voucher[]>([]);
  const [selectedVoucher, setSelectedVoucher] = useState<string>("");
  const [appliedDiscount, setAppliedDiscount] = useState<number>(0);

  const location = useLocation();
  const queryParams = new URLSearchParams(location.search);

  const user_id = queryParams.get("user_id");

  useEffect(() => {
    const fetchCartItems = async () => {
      try {
        const items = await invoke<any[]>("get_cart_items", {
          userId: user_id,
        });
        console.log("Cart Items:", items);
        setCartItems(items);
      } catch (error) {
        console.error("Failed to fetch cart items:", error);
      }
    };

    fetchCartItems();
  }, []);

  useEffect(() => {
    const fetchVoucher = async () => {
      try {
        const menuNames = [...new Set(cartItems.map((item) => item.menu_name))];
        const voucherPromises = menuNames.map((menuName) =>
          invoke<Voucher[]>("get_menu_vouchers", {
            menuName: menuName,
          })
        );
        const voucherResults = await Promise.all(voucherPromises);
        const allVouchers = voucherResults.flat();

        console.log("Available Vouchers:", allVouchers);
        setVoucher(allVouchers);
      } catch (error) {
        console.error("Failed to fetch vouchers:", error);
      }
    };

    if (cartItems.length > 0) {
      fetchVoucher();
    }
  }, [cartItems]);

  const handleVoucherSelect = (code: string) => {
    if (!code) {
      setAppliedDiscount(0);
      setSelectedVoucher("");
      return;
    }

    const selectedVoucherItem = voucher.find((v) => v.code === code);
    if (selectedVoucherItem) {
      const discountedItem = cartItems.find(
        (item) => item.menu_name === selectedVoucherItem.menu_name
      );
      if (discountedItem) {
        const discountAmount =
          (discountedItem.total_price * selectedVoucherItem.discount_percent) /
          100;
        setAppliedDiscount(discountAmount);
        setSelectedVoucher(code);
      }
    }
  };

  // const handleCheckout = async () => {
  //   try{
  //     if(cartItems.length === 0){
  //       alert("Your cart is empty. Please add items to your cart before checking out.");
  //       return;
  //     }
  //   }

  //   const totalAmount = cartItems.reduce(
  //     (sum, item) => sum + (item.total_price || 0), 0
  //   ) - appliedDiscount;

  //   if(selectedVoucher) {
  //     await invoke("apply_menu_voucher", {
  //       code: selectedVoucher,
  //       menuName: cartItems.find(item => item.menu_name === voucher.find(v => v.code === selectedVoucher)?.menu_name)?.menu_name,
  //     });
  //   }

  // }

  const incrementQuantity = (index: number) => {
    const updatedCart = [...cartItems];
    updatedCart[index].quantity += 1;
    updatedCart[index].total_price =
      (updatedCart[index].price || 0) * updatedCart[index].quantity;
    setCartItems(updatedCart);
  };

  const decrementQuantity = (index: number) => {
    const updatedCart = [...cartItems];
    if (updatedCart[index].quantity > 1) {
      updatedCart[index].quantity -= 1;
      updatedCart[index].total_price =
        (updatedCart[index].price || 0) * updatedCart[index].quantity;
      setCartItems(updatedCart);
    }
  };

  const removeItem = async (index: number) => {
    try {
      const updatedCart = cartItems.filter((_, i) => i !== index);
      setCartItems(updatedCart);

      await invoke("update_cart", {
        request: {
          user_id: user_id,
          items: updatedCart,
        },
      });

      alert("Item removed from cart successfully!");
    } catch (error) {
      console.error("Failed to remove item:", error);
      alert("Failed to remove item from cart.");
    }
  };

  const saveCart = async () => {
    try {
      if (!user_id) {
        alert("User ID is not available. Please log in again.");
        return;
      }

      await invoke("update_cart", {
        request: {
          user_id: user_id,
          cart_items: cartItems.map((item) => ({
            menu_name: item.menu_name,
            branch_address: item.branch_address,
            price: item.price,
            menu_type: item.menu_type,
            quantity: item.quantity,
          })),
        },
      });
      alert("Update Cart Berrhasil!");
    } catch (error) {
      console.error("Gagal:", error);
      alert("Failed to save cart. Please try again.");
    }
  };

  const handleCheckout = async () => {
    try {
      if (cartItems.length === 0) {
        alert(
          "Your cart is empty. Please add items to your cart before checking out."
        );
        return;
      }

      // Calculate subtotal first
      const subtotal = cartItems.reduce(
        (sum, item) => sum + (Number(item.total_price) || 0),
        0
      );

      // Calculate final total with discount
      const finalTotal = subtotal - (appliedDiscount || 0);

      // Validate total amount
      if (isNaN(finalTotal) || finalTotal < 0) {
        throw new Error("Invalid total amount calculated");
      }

      if (selectedVoucher) {
        await invoke("apply_menu_voucher", {
          code: selectedVoucher,
          menuName: cartItems.find(
            (item) =>
              item.menu_name ===
              voucher.find((v) => v.code === selectedVoucher)?.menu_name
          )?.menu_name,
        });
      }

      await invoke("create_transaction", {
        request: {
          user_id: parseInt(user_id || "0"),
          cart_items: cartItems.map((item) => ({
            menu_name: item.menu_name,
            branch_address: item.branch_address,
            price: Number(item.price) || 0,
            menu_type: item.menu_type,
            quantity: Number(item.quantity) || 0,
          })),
          total_amount: Number(finalTotal),
          applied_voucher: selectedVoucher || null,
          discount_amount: Number(appliedDiscount || 0),
        },
      });

      // Clear cart after successful transaction
      await invoke("update_cart", {
        request: {
          user_id: user_id,
          cart_items: [],
        },
      });

      setCartItems([]);
      setSelectedVoucher("");
      setAppliedDiscount(0);
      alert("Checkout successful! Your order has been placed.");
    } catch (error) {
      console.error("Checkout failed:", error);
      alert("Failed to proceed with checkout. Please try again.");
    }
  };

  return (
    <div className="bg-slate-300 min-h-screen p-0 m-0">
      <NavigationBar></NavigationBar>
      <h1 className="text-3xl font-bold text-center mb-6">Your Cart</h1>

      <div className="flex">
        <div className="flex-1">
          {cartItems.length === 0 ? (
            <p className="text-center text-gray-500">Your cart is empty.</p>
          ) : (
            cartItems.map((item, index) => (
              <div
                key={index}
                className="bg-white shadow-md rounded-lg p-4 mb-4 flex justify-between items-center"
              >
                <div>
                  <h2 className="text-lg font-bold">{item.menu_name}</h2>
                  <p className="text-gray-600">Branch: {item.branch_name}</p>
                  <p className="text-gray-600">
                    Price: ${item.price?.toFixed(2) || "0.00"}
                  </p>
                  <p className="text-gray-600">
                    Total: ${item.total_price?.toFixed(2) || "0.00"}
                  </p>
                </div>
                <div className="flex items-center space-x-2">
                  <button
                    onClick={() => decrementQuantity(index)}
                    className="bg-red-500 text-white px-3 py-1 rounded"
                  >
                    -
                  </button>
                  <span className="px-4 py-2 bg-gray-100 border">
                    {item.quantity}
                  </span>
                  <button
                    onClick={() => incrementQuantity(index)}
                    className="bg-green-500 text-white px-3 py-1 rounded"
                  >
                    +
                  </button>
                  <button
                    onClick={() => removeItem(index)}
                    className="bg-gray-500 text-white px-3 py-1 rounded"
                  >
                    Remove
                  </button>
                </div>
              </div>
            ))
          )}
        </div>

        <div className="w-1/4 bg-white shadow-md rounded-lg p-4 ml-4">
          <h2 className="text-xl font-bold mb-4">Checkout</h2>

          {voucher.length > 0 && (
            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Available Vouchers
              </label>
              <select
                value={selectedVoucher}
                onChange={(e) => handleVoucherSelect(e.target.value)}
                className="w-full border rounded-md py-2 px-3 text-gray-700"
              >
                <option value="">Select a voucher</option>
                {voucher.map((v) => (
                  <option key={v.id} value={v.code}>
                    {v.code} - {v.discount_percent}% off {v.menu_name}
                  </option>
                ))}
              </select>
            </div>
          )}

          <p className="text-gray-600 mb-4">
            Total Items:{" "}
            {cartItems.reduce((sum, item) => sum + (item.quantity || 0), 0)}
          </p>

          <p className="text-gray-600 mb-2">
            Subtotal: $
            {cartItems
              .reduce((sum, item) => sum + (item.total_price || 0), 0)
              .toFixed(2)}
          </p>

          {appliedDiscount > 0 && (
            <p className="text-green-600 mb-2">
              Discount: -${appliedDiscount.toFixed(2)}
            </p>
          )}

          <p className="text-lg font-bold mb-4">
            Final Total: $
            {(
              cartItems.reduce(
                (sum, item) => sum + (item.total_price || 0),
                0
              ) - appliedDiscount
            ).toFixed(2)}
          </p>

          <button
            onClick={handleCheckout}
            className="bg-blue-500 text-white w-full py-2 rounded hover:bg-blue-600"
          >
            Proceed to Checkout
          </button>

          <button
            onClick={saveCart}
            className="bg-green-500 text-white w-full py-2 rounded mt-2 hover:bg-green-600"
          >
            Save Cart
          </button>
        </div>
      </div>
    </div>
  );
}

export default CartPage;
