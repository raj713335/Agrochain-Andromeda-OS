import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import AOSFarmer from '../../Components/AOSFarmer';
import AOSMintList from '../../Components/AOSMintList';
import { NftContext } from '../../frontend/NftContext/NftProvider';

const mockCtx = (overrides) => ({
  aosClient: { client: { execute: jest.fn() }, signerAddress: 'andr1abc' },
  carbonAdoAddress: 'andr1xxx',
  setCarbonAdoAddress: jest.fn(),
  ...overrides
});

test('farmer registration triggers execute', async () => {
  const ctx = mockCtx();
  render(
    <NftContext.Provider value={ctx}>
      <AOSFarmer />
    </NftContext.Provider>
  );
  fireEvent.change(screen.getByPlaceholderText(/Name/i), { target: { value: 'John' } });
  fireEvent.change(screen.getByPlaceholderText(/Location/i), { target: { value: 'CA' } });
  fireEvent.change(screen.getByPlaceholderText(/Land Area/i), { target: { value: '10' } });
  fireEvent.change(screen.getByPlaceholderText(/Contact/i), { target: { value: 'x@y.z' } });
  fireEvent.change(screen.getByPlaceholderText(/IoT Device Id/i), { target: { value: 'dev1' } });
  fireEvent.change(screen.getByPlaceholderText(/Practices/i), { target: { value: 'organic' } });
  fireEvent.click(screen.getByText(/Register/i));
});

test('mint and list execute without crash', async () => {
  const ctx = mockCtx();
  render(
    <NftContext.Provider value={ctx}>
      <AOSMintList />
    </NftContext.Provider>
  );
  fireEvent.change(screen.getByPlaceholderText(/Amount/i), { target: { value: '1000' } });
  fireEvent.change(screen.getByPlaceholderText(/IoT Device Id/i), { target: { value: 'dev1' } });
  fireEvent.change(screen.getByPlaceholderText(/Location/i), { target: { value: 'CA' } });
  fireEvent.click(screen.getByText(/^Mint$/));

  fireEvent.change(screen.getByPlaceholderText(/Credit Id/i), { target: { value: 'credit_1' } });
  fireEvent.change(screen.getByPlaceholderText(/Price \(uandr\)/i), { target: { value: '10' } });
  fireEvent.change(screen.getByPlaceholderText(/Quantity/i), { target: { value: '2' } });
  fireEvent.click(screen.getByText(/^List$/));
});


