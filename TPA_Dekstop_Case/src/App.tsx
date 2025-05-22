import { useEffect, useState } from "react";
import { Link } from "react-router";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { app } from "@tauri-apps/api";
import NavigationBar from "./Navbarnya/Navbar";

function App(){

  const [username, setText] = useState("");
  const [password, setText2] = useState("");

  type LoginResponse = {
    message: string,
    token: string,
  };

  // useEffect(() => {
  //             const fetchUserData = async () => {
  //                 try{
  //                     const response= await invoke<string>("start_fetch");
  //                     console.log("Halo");
  //                     const data = JSON.parse(response);
  //                     console.log(data);
      
  //                 } catch (error){
  //                     console.error("Error fetching user data:", error);
  //                 }
  //             }
      
  //             fetchUserData();
  //     }, []);

  async function checkUserSession(token: string) {
    try {
        const response = await invoke("check_session_handler", { token });
        console.log("Session status:", response);
    } catch (error) {
        console.error("Error checking session:", error);
    }
  }

  const userToken = localStorage.getItem("user_token");
  if (userToken) {
      checkUserSession(userToken);
  }

  const ketikabuttonlogindipencet = async () => {

    if(!username || !password){
      alert("Isi Woi");
      return;
    }

    try {
      const response = await invoke<string>('login_user', {
        username : username, password : password
      })
      console.log("Login response:", response);
      localStorage.setItem("sessionToken", response);
      // alert(response);
      window.location.href = "/mainpage";
    } catch (error) {
      alert("Gabisa Woi " + error);
    }
  }


  async function checkSession() {
    const token = localStorage.getItem("sessionToken");
    if (!token) {
        console.log("Belum login");
        return;
    }

    try {
        const response = await invoke("check_session_handler", { token });
        console.log(response);
    } catch (error) {
        console.error("Session error:", error);
    }
  }

  checkSession()

  return <div className="bg-gray-900 text-white min-h-screen flex justify-center items-center">
    <div className="flex flex-col gap-10 justify-center items-center h-screen">
    <img src="src\img\DALL·E 2025-03-28 02.27.41 - A modern and elegant logo for 'YoshiKoya' restaurant. The design incorporates Japanese elements such as a stylized ramen bowl with chopsticks, subtle .webp" alt="" className="w-32 h-32 rounded-md"/>
      <div className="w-80 h-100 gap-5 p-3 bg-slate-300 text-black flex flex-col justify-center items-center rounded-lg border">
      <div className="flex justify-between">
          <label className="text-black font-bold">Masukan Username</label>
        </div>
        <input type="text" value={username} onChange={(e) => setText(e.target.value)} placeholder="Masukan Username" className="border-2 px-2 rounded-md w-full"></input>
        <div className="flex justify-between">
          <label className="text-black font-bold">Masukan Password</label>
        </div>
        <input type="password" value={password} onChange={(e) => setText2(e.target.value)} placeholder="Masukan Password" className="border-2 px-2 rounded-md w-full"></input>
        <button className="bg-white text-black font-bold px-6 py-2 rounded-md hover:bg-gray-200 transition " onClick={ketikabuttonlogindipencet}>LOGIN</button>
        <Link to="/register">
          <button className="text-blue-400 px-4 py-2 rounded-md hover:text-blue-500 underline-offset-1 transition">
              Belum Punya Akun? Click disini
          </button>
        </Link>
      </div>
    </div>
  </div>
}

export default App