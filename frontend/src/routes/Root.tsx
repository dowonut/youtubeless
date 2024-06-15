import "./Root.css";
import Navbar from "../components/Navbar/Navbar.tsx";
import { Outlet } from "react-router-dom";
import Footer from "../components/Footer/Footer.tsx";

function Root() {
  return (
    <>
      <Navbar></Navbar>
      <div id="content">
        <Outlet></Outlet>
      </div>
      <Footer></Footer>
    </>
  );
}

export default Root;
