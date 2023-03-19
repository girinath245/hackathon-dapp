import React from 'react'
import './Card.css';
import { useNavigate } from 'react-router-dom';

const Card = () => {
    const navigate = useNavigate();

    const redirectToProject = () => {
        navigate('/project');
    }

    return (
        <div className='card-component' onClick={redirectToProject}>
            <h3>Title</h3>
            <div>
                <span className='card-title-name'>
                    Start:
                </span>
                <span className='card-title-value'>
                    5345543543534
                </span>
            </div>

            <div>
                <span className='card-title-name'>
                    End:
                </span>
                <span className='card-title-value'>
                    5345543543534
                </span>
            </div>

            <div>
                <span className='card-title-name'>
                    Status:
                </span>
                <span className='card-title-value'>
                    5345543543534
                </span>
            </div>

            <div>
                <span className='card-title-name'>
                    Manager Name:
                </span>
                <span className='card-title-value'>
                    5345543543534
                </span>
            </div>

            <div>
                <span className='card-title-name'>
                    Manager Address:
                </span>
                <span className='card-title-value'>
                    5345543543534
                </span>
            </div>

            <div>
                <span className='card-title-name'>
                    Id:
                </span>
                <span className='card-title-value'>
                    5345543543534
                </span>
            </div>
        </div>
    )
}

export default Card;