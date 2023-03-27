import { Navlink, Outlet } from 'react-router-dom';
const About = () => {
    return (
        <>
            <h1>About page.</h1>
            <div className="about">
                <ul className='about_list'>
                    <li>
                        <NavLink to="about-app">About app</NavLink>
                    </li>
                    <li>
                        <Navlink to="about-developer">About developer</Navlink>
                    </li>
                </ul>
                <Outlet />
            </div>
        </>
    )
}
export default About;