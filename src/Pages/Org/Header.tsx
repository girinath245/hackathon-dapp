import { useState } from 'react';
import logo from '../smart-contracts.png';

import './Header.css'

interface addressProps {
  addr : string ;
};

function Header() {
  return (
    <>
        <div className='org-header'>
                <div className='header-logo-name'>
                    <img src={logo} alt='org'/>
                    <span className='header-name'>Toyota</span>
                </div>
                <div className='header-guest'>
                    <span style={{fontWeight: 700}}>Hello, </span>
                    <span className='header-guest-name'> Guest </span>
                </div>
            </div>
    </>
  )
}

export default Header;