import React, { useContext, useState } from 'react';
import { NftContext } from '../frontend/NftContext/NftProvider';

export default function AOSFarmer() {
    const { aosClient, carbonAdoAddress, setCarbonAdoAddress } = useContext(NftContext);
    const [form, setForm] = useState({ name: '', location: '', land_area: '', contact: '', iot_device_id: '', practices: '' });
    const [busy, setBusy] = useState(false);
    const onChange = (e) => setForm({ ...form, [e.target.name]: e.target.value });

    const register = async () => {
        if (!aosClient) return;
        setBusy(true);
        try {
            const msg = {
                register_farmer: {
                    farmer_data: {
                        name: form.name,
                        location: form.location,
                        land_area: form.land_area,
                        contact: form.contact,
                        iot_device_id: form.iot_device_id,
                        farming_practices: form.practices.split(',').map(s => s.trim()).filter(Boolean)
                    }
                }
            };
            await aosClient.client.execute(aosClient.signerAddress, carbonAdoAddress, msg, 'auto');
            alert('Registered on aOS');
        } catch (e) {
            console.error(e);
            alert('Failed');
        } finally {
            setBusy(false);
        }
    };

    return (
        <div className="container mt-4">
            <h2>aOS Farmer</h2>
            <div className="card p-3 mb-3">
                <label>Carbon ADO Address</label>
                <input className="form-control" value={carbonAdoAddress} onChange={(e)=>setCarbonAdoAddress(e.target.value)} placeholder="andr1..." />
            </div>
            <div className="card p-3">
                <div className="row g-2">
                    <div className="col-md-6"><input className="form-control" name="name" placeholder="Name" onChange={onChange} /></div>
                    <div className="col-md-6"><input className="form-control" name="location" placeholder="Location" onChange={onChange} /></div>
                    <div className="col-md-6"><input className="form-control" name="land_area" placeholder="Land Area" onChange={onChange} /></div>
                    <div className="col-md-6"><input className="form-control" name="contact" placeholder="Contact" onChange={onChange} /></div>
                    <div className="col-md-6"><input className="form-control" name="iot_device_id" placeholder="IoT Device Id" onChange={onChange} /></div>
                    <div className="col-md-6"><input className="form-control" name="practices" placeholder="Practices (comma separated)" onChange={onChange} /></div>
                </div>
                <button disabled={busy || !aosClient} className="btn btn-primary mt-3" onClick={register}>{busy ? 'Submitting...' : 'Register'}</button>
            </div>
        </div>
    );
}


