import React from 'react'
import './ListRow.css'

function ListRow() {
  return (
    <div className='list-row'>
      <div className='list-row-wrapper'>
        <p className='list-row-wrapper-name'>
          Serial No.
        </p>
        <p className='serial-number'>
          1
        </p>
      </div>
      <div className='list-row-wrapper'>
        <p className='list-row-wrapper-name'>
          Name
        </p>
        <p className='name'>
          Sarnavo Sarkar
        </p>
      </div>
      <div className='list-row-wrapper'>
        <p className='list-row-wrapper-name'>
          Comany Id
        </p>
        <p className='company-id'>
          Google
        </p>
      </div>
      <div className='list-row-wrapper'>
        <p className='list-row-wrapper-name'>
          Desgination
        </p>
        <p className='Designation'>
          Manager
        </p>
      </div>
      <div className='list-row-wrapper'>
        <p className='list-row-wrapper-name'>
          Rating          
        </p>
        <p className='rating'>
          88
        </p>
      </div>
    </div>
  )
}

export default ListRow