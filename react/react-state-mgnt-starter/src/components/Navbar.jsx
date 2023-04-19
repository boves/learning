import { NavLink } from "react-router-dom";
import { useAuthContext } from "@/context/AuthContext";
import React, { useState } from 'react';

const links = [
    { path: '/', text: 'Home' },
    { path: 'about', text: 'About' },
    { path: 'profile', text: 'Profile' },
    { path: 'login', text: 'Login' },
];
const Navbar = () => {
    const [navbarOpen, setNavbarOpen] = useState(false);
    const {user, logout } = useAuthContext();
    const handleLogout = () => {
      logout();
    };
    return (
      <>
        <nav className="navbar">
          <button
            className="toggle"
            onClick={() => setNavbarOpen((prev) => !prev)}
          >
            {navbarOpen ? 'close': 'open'}
          </button>
        <ul className={`menu-nav${navbarOpen ? ' show-menu' : ''}`}>
          {links.map((link) => {
            return (
              <React.Fragment key={link.text}>
                {link.path === 'login' ? (
                  !user && (
                    <li>
                      <NavLink to={link.path}>{link.text}</NavLink>
                    </li>
                  )
                ) : link.path === 'profile' ? (
                  user && (
                    <li>
                      <NavLink to={link.path}>
                        {link.text}
                      </NavLink>
                    </li>
                  )
                ) : (
                  <li>
                    <NavLink to={link.path}>{link.text}</NavLink>
                  </li>
                )}
              </React.Fragment>
            );
          })}
        </ul>

        </nav>
        {user && (
          <div className="logout">
            <p>{user}</p>
            {<button onClick={handleLogout}>Logout</button>}
          </div>
        )}
      </>
    )
};
export default Navbar;