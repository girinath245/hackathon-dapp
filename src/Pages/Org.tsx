import './Org.css'
import {userContextOrg} from './Org/Header';
import Header from './Org/Header';
import SidePanel from './Org/SidePanel';

import { useLocation } from 'react-router-dom';
  
const Org = () => {
    const location = useLocation();
    console.log("User is ",location.state.accountaddress);

    return (
        <>
            <userContextOrg.Provider value={location.state.accountaddress}>
            <Header />
            </userContextOrg.Provider>
            <SidePanel />
        </>
    );
};

export default Org;