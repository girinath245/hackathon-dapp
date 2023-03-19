import { useContext, createContext } from 'react';
import logo from '../smart-contracts.png';

import './Header.css'

export const userContextProj = createContext("Guest");

const Header = () => {
  
  const username= useContext(userContextProj);

  return (
    <>
        <div className='org-header'>
                <div className='header-logo-name'>
                    <img src={logo} alt='org'/>
                    <span className='header-name'>Toyota</span>
                </div>
                <div className='header-guest'>
                    <span style={{fontWeight: 700}}>Hello, </span>
                    <span className='header-guest-name'> {username} </span>
                </div>
            </div>
    </>
  )
};

export default Header;