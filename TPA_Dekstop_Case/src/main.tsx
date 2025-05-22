import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { BrowserRouter, Route, Routes } from "react-router";
// import LoginPage from "./Pages/LoginPage";
import RegisterPage from "./Pages/RegisterPage";
import MainPage from "./Pages/MainPage";
import PurchasePage from "./Pages/PurchasePage";
import CartPage from "./Pages/Cart";
import SetOperationPage from "./Pages/SetOperationPage";
import ReservationPage from './Pages/ReservationPage';
import ChooseBranch from "./Pages/ChooseBranch";
import OpenBranchPage from "./Pages/OpenBranch";
import BranchList from "./Pages/BranchList";
import BranchDetail from "./Pages/BranchDetailPage";
import CreateVoucherPage from "./Pages/CreateVoucherPage";
import HRDashboardPage from "./Pages/HRDashboardPage";
import JobApplicationPage from "./Pages/ApplicantPage";
import EmployeeManagement from "./Pages/EmployeeManagementPage";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <BrowserRouter>
    <Routes>
      <Route path="/" element={<App></App>}></Route>
      {/* <Route path="/login" element={<LoginPage></LoginPage>}></Route> */}
      <Route path="/register" element={<RegisterPage></RegisterPage>}></Route>
      <Route path="/mainpage" element={<MainPage></MainPage>}></Route>
      <Route path="/purchase" element={<PurchasePage></PurchasePage>}></Route>
      <Route path="/cart" element={<CartPage></CartPage>}></Route>
      <Route path="/setoperation" element={<SetOperationPage></SetOperationPage>}></Route>
      <Route path="/reservation" element={<ReservationPage></ReservationPage>}></Route>
      <Route path="/choosebranch" element={<ChooseBranch></ChooseBranch>}></Route>
      <Route path="/openbranch" element={<OpenBranchPage></OpenBranchPage>}></Route>
      <Route path="/viewallbranch" element={<BranchList></BranchList>}></Route>
      <Route path="/branchdetail" element={<BranchDetail></BranchDetail>}></Route>
      <Route path="/create-voucher" element={<CreateVoucherPage/>} />
      <Route path="/apply-job" element={<JobApplicationPage></JobApplicationPage>} />
      <Route path="/hr-dashboard" element={<HRDashboardPage></HRDashboardPage>} />
      <Route path="/employee-management" element={<EmployeeManagement></EmployeeManagement>} />
    </Routes>
  </BrowserRouter>
);
