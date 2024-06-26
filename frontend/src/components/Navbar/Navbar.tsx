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
            <FontAwesomeIcon className="icon" icon={faRss}></FontAwesomeIcon>
            Feed
          </NavLink>
        </div>
        <div>
          <NavLink className="link" to="/search">
            <FontAwesomeIcon className="icon" icon={faMagnifyingGlass}></FontAwesomeIcon>
            Search
          </NavLink>
        </div>
        <div>
          <NavLink className="link" to="/profile">
            <FontAwesomeIcon className="icon" icon={faUser}></FontAwesomeIcon>
            Profile
          </NavLink>
        </div>
      </div>
    </>
  );
}

export default Navbar;
