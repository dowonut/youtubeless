import "./Navbar.css";
import { NavLink } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faMagnifyingGlass, faRss, faUser } from "@fortawesome/free-solid-svg-icons";

function Navbar() {
  return (
    <>
      <div id="navbar">
        <div>
          <NavLink className="link" to="/">
            <FontAwesomeIcon icon={faRss}></FontAwesomeIcon>
            Feed
          </NavLink>
        </div>
        <div>
          <NavLink className="link" to="/search">
            <FontAwesomeIcon icon={faMagnifyingGlass}></FontAwesomeIcon>
            Search
          </NavLink>
        </div>
        <div>
          <NavLink className="link" to="/profile">
            <FontAwesomeIcon icon={faUser}></FontAwesomeIcon>
            Profile
          </NavLink>
        </div>
      </div>
    </>
  );
}

export default Navbar;
