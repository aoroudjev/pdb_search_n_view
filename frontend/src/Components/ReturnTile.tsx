import React, { FC } from "react";
import './ReturnTile.css'

interface ReturnTileProps {
    result: {
        title: string;
        description: string;
    };
}

const ReturnTile: FC<ReturnTileProps> = ({ result }) => {
    return(
        <div className="resultTile">
            <h2>{result.title}</h2>
            <p>{result.description}</p>
        </div>
    );
};

export default ReturnTile;