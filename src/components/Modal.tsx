import React from "react";

interface ModalProps {
    show: boolean;
    onClose: () => void;
    children: React.ReactNode;
}

export function Modal({ show, onClose, children }: ModalProps) {
    if (!show) {
        return null;
    }

    return (
        <div className="modal-overlay">
            <div className="modal">
                <button className="close-button" onClick={onClose}>X</button>
                {children}
            </div>
        </div>
    );
}