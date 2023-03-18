import React from 'react'
import Card from './Card'
import Header from './Header'
import SidePanel from './SidePanel'

import './Projects.css';

function Projects() {
  return (
    <>
        <Header />
        <SidePanel />
        <div className='card-collection'>
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
        </div>
    </>
  )
}

export default Projects