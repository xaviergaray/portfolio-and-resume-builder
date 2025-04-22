'use client';

import { useState, useEffect } from 'react';

type BlobData = {
    id: string;
    size: number;
    top: number;
    left: number;
    colorClass: string;
    animationClass: string;
    opacity: number;
};

export default function BackgroundBlobs({ count = 5 }: { count?: number; }) {
    const [blobs, setBlobs] = useState<BlobData[]>([]);

    useEffect(() => {
        const viewHeight = window.innerHeight;

        const pageWidth = window.innerWidth;
        const colors    = [
            'bg-pink-400 dark:bg-pink-600',
            'bg-blue-300 dark:bg-blue-700',
            'bg-purple-400 dark:bg-purple-600',
            'bg-green-300 dark:bg-green-600',
        ];
        const anims     = ['blob', 'blob-reverse'];

        const additions: BlobData[] = [];

        for (let i = 0; i < count; i++) {
            additions.push({
                id:             `b-${i}`,
                size:           120 + Math.random() * 200,
                top:            Math.random() * viewHeight,
                left:           Math.random() * pageWidth,
                colorClass:     colors[Math.floor(Math.random() * colors.length)],
                animationClass: anims[Math.floor(Math.random() * anims.length)],
                opacity:        0.3 + Math.random() * 0.5,
            });
        }

        setBlobs((prev) => [...prev, ...additions]);
    }, [count]);

    return (
        <div className="fixed inset-0 overflow-hidden pointer-events-none -z-10" aria-hidden>
            {blobs.map((b) => (
                <div
                    key={b.id}
                    className={`absolute rounded-full ${b.colorClass} ${b.animationClass}`}
                    style={{
                        width:   `${b.size}px`,
                        height:  `${b.size}px`,
                        top:     `${b.top}px`,
                        left:    `${b.left}px`,
                        opacity: b.opacity,
                    }}
                />
            ))}
        </div>
    );
}
