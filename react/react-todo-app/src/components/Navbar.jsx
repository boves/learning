import { useState, useRef } from "react";

const Navbar = () => {
    console.log(useState(false));
    const [dropbdown, setDropdown] = useState(false);
    const ref = useRef();

    useEffect(() => {
        const handler = (event) =>{
            if (dropbdown && ref.current && !ref.current.contains(event.target)) {
                setDropdown(false);
            }
        };
        document.addEventListener("mousedown", handler);
        return () => {
            document.removeEventListener("mousedown", handler);
        };
    }, [dropbdown]);

    return (
        <nav>
            <ul>
                <li>Home</li>
                <li>About</li>
                <li ref={ref}>
                    <button onClick={() => setDropdown((prev) => !prev)}>
                        Services <span>&#8595;</span>
                    </button>
                    {dropbdown && (
                        <ul>
                            <li>Design</li>
                            <li>Development</li>
                        </ul>
                    )}
                </li>
            </ul>
        </nav>
    );
};
export default Navbar;
