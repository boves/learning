import { Routes, Route } from 'react-router-dom';

import Home from '@/routes/Home';
import About from '@/routes/About';
import Login from '@/routes/Login';
import Profile from '@/routes/Profile';
import NotMatch from '@/routes/NotMatch';
import Layout from '@/components/Layout';
import SinglePage from '@/routes/SinglePage'
import ProtectedRoute from '@/components/ProtectedRoute';

const TodoApp = () => {
    return (
        <Routes path ="/" element={<Layout />}>
            <Route index element={<Home />} />
            <Route path="about" element={<About />} />
            <Route path="login" element={<Login />} />
            <Route path="profile" element={<Profile />} />
            <Route path="*" element={<NotMatch />} />
            <ProtectedRoute>
                <Profile />
            </ProtectedRoute>
        </Routes>
    );
};
export default TodoApp;
