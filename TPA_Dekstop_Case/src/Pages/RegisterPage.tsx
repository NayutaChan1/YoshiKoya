import { useEffect, useState } from "react";
import { Link } from "react-router";
import "../App.css";
import { invoke } from "@tauri-apps/api/core";

const RegisterPage = () => {
    
    const [username, setText] = useState("");
    const [password, setText2] = useState("");
    const [email, setText3] = useState("");

    // useEffect(() => {
    //         const fetchUserData = async () => {
    //             try{
    //                 const response= await invoke<string>("start_fetch");
    //                 console.log("Halo");
    //                 const data = JSON.parse(response);
    //                 console.log(data);
    
    //             } catch (error){
    //                 console.error("Error fetching user data:", error);
    //             }
    //         }
    
    //         fetchUserData();
    // }, []);

    const ketikabuttondipencet = async () => {
      if(!username || !password || !email){
        alert("Isi Woi");
        return;
      }else if(!email.includes('@')){
        alert("Email harus ada '@'nya woi");
        return;
      }

      try{
        const response = await invoke("register_user", {
          username, email, password
        });

        // alert(response);
        window.location.href = "/";
      }catch(error){
        alert("Gabisa Woi " + error);
      }

    }


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
        <div className="flex justify-between">
          <label className="text-black font-bold">Masukan Email</label>
        </div>
        <input type="text" value={email} onChange={(e) => setText3(e.target.value)} placeholder="Masukan Email" className="border-2 px-2 rounded-md w-full"></input>
        <button className="bg-white text-black font-bold px-6 py-2 rounded-md hover:bg-gray-200 transition " onClick={ketikabuttondipencet}>REGISTER</button>
        <Link to="/">
          <button className="text-blue-400 px-4 py-2 rounded-md hover:text-blue-500 underline-offset-1 transition">
              Sudah Punya Akun? Click disini
          </button>
        </Link>
      </div>
    </div>
  </div>
}

export default RegisterPage