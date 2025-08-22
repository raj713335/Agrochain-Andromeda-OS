import React, { useContext, useEffect, useState } from 'react';
import { NftContext } from '../frontend/NftContext/NftProvider';

export default function AOSStats() {
    const { aosClient, carbonAdoAddress } = useContext(NftContext);
    const [stats, setStats] = useState(null);
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        const run = async () => {
            if (!aosClient || !carbonAdoAddress) return;
            setLoading(true);
            try {
                const res = await aosClient.client.queryContractSmart(carbonAdoAddress, { get_global_stats: {} });
                setStats(res.stats || res);
            } catch (e) { console.error(e); }
            setLoading(false);
        };
        run();
    }, [aosClient, carbonAdoAddress]);

    if (!aosClient) return <div className="container mt-4">Connect Keplr to view stats.</div>;
    if (loading) return <div className="container mt-4">Loading...</div>;

    return (
        <div className="container mt-4">
            <h2>aOS Global Stats</h2>
            {!stats && <div>No stats</div>}
            {stats && (
                <div className="card p-3">
                    <div>Total Credits Minted: {stats.total_credits_minted}</div>
                    <div>Total Credits Sold: {stats.total_credits_sold}</div>
                    <div>Active Listings: {stats.active_listings}</div>
                    <div>Average Carbon Score: {stats.average_carbon_score}</div>
                </div>
            )}
        </div>
    );
}


