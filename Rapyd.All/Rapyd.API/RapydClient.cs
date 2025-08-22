using System;
using System.Collections.Generic;
using System.Linq;
using System.Security.Cryptography;
using System.Security.Cryptography.Xml;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;
using Agrochain.API.Dto;
using Agrochain.API.Dto.Response;
using RestSharp;

namespace Agrochain.API
{
    public class RapydClient
    {
        private readonly string _accessKey;
        private readonly string _secretKey;
        private readonly string _baseUrl;
        private readonly RestClient _client;

        public RapydClient(IConfiguration configuration)

        {
            _accessKey = configuration.GetValue<string>("AccessKey");
            _secretKey = configuration.GetValue<string>("SecretKey");
            _baseUrl = configuration.GetValue<string>("BaseRapydApiUrl");
            _client = new RestClient(_baseUrl);
        }

        #region Card related
        public async Task<CardIssueResponse> IssueCard(CardIssue cardDetails)
        {
            return await MakeRequest<CardIssueResponse>(Method.Post, "/v1/issuing/cards", cardDetails);
        }

        public async Task<CardIssueResponse> ActivateIssuedCard(string cardId)
        {
            return await MakeRequest<CardIssueResponse>(Method.Post, "/v1/issuing/cards/activate", cardId);
        }

        public async Task<ListOfIssuedCards> ListOfIssueCard(string walletId, int pagenumber = 0, int pageSize = 0)
        {
            return await MakeRequest<ListOfIssuedCards>(Method.Post, $"/v1/issuing/cards?{walletId}&{pagenumber}&{pageSize}");
        }

        public async Task<CardIssueResponse> GetIssuedCardById(string cardId)
        {
            return await MakeRequest<CardIssueResponse>(Method.Get, $"/v1/issuing/cards?{cardId}");
        }
        #endregion

        #region Wallet
        public async Task<bool> EnableWallet(object ewallet = null, string phone_number = null)
        {
            var apiResponse = await MakeRequest<BaseApiResponse>(Method.Post, $"/v1/user/enable", ewallet ?? phone_number);
            return apiResponse.Status.OperationStatus == "SUCCESS";
        }

        public async Task<ListOfWallet> GetListOfWallet(string type, Guid referenceId, int pagenumber = 0, int pageSize = 0)
        {
            return await MakeRequest<ListOfWallet>(Method.Get, $"/v1/user/wallets?{type}&{referenceId}&{pagenumber}&{pageSize}");
        }

        public async Task<CreateWalletResponse> CreateNewWallet(string walletDetails)
        {
            var apiResponse = await MakeRequest<CreateWalletResponse>(Method.Post, "/v1/user", walletDetails);
            return apiResponse;
        }
        #endregion

        #region customer related
        public async Task<List<Customer>> GetCustomers()
        {
            var apiResponse = await MakeRequest<Response<List<Customer>>>(Method.Get, "/v1/customers");
            var result = apiResponse;
            return result?.Data;
        }

        public async Task<Customer> CreateCustomer(CreateCustomerBody body)
        {
            var apiResponse = await MakeRequest<Response<Customer>>(Method.Post, "/v1/customers", body);
            var result = apiResponse;
            return result?.Data;
        }
        #endregion

        #region payment related
        public async Task<Checkout> CreatePaymentCheckout(CreateCheckoutBody body)
        {
            var apiResponse = await MakeRequest<Response<Checkout>>(Method.Post, "/v1/checkout", body);
            var result = apiResponse.Data;
            return result;
        }

        //public async Task<List<PaymentMethod>> GetPaymentMethods(string country)
        //{
        //    var apiResponse = await MakeRequest<Response<List<PaymentMethod>>>(Method.Get, $"/v1/payment_methods/country?country={country}");
        //    var result = apiResponse.Data;
        //    return result;
        //}

        //public async Task<PaymentMethodRequiredFields> GetPaymentMethodRequiredFields(string type)
        //{
        //    var apiResponse = await MakeRequest<Response<PaymentMethodRequiredFields>>(Method.Get, $"/v1/payment_methods/required_fields/{type}");
        //    var result = apiResponse.Data;
        //    return result;
        //}

        public async Task<CardPaymentResponse> CreatePayment(CardPayment body)
        {
            var apiResponse = await MakeRequest<Response<CardPaymentResponse>>(Method.Post, $"/v1/payments", body);
            var result = apiResponse.Data;
            return result;
        }

        public async Task<Payment> CancelPayment(string paymentId)
        {
            var apiResponse = await MakeRequest<Response<Payment>>(Method.Delete, $"/v1/payments/{paymentId}");
            var result = apiResponse.Data;
            return result;
        }

        public async Task<List<CustomerPaymentMethod>> GetCustomersPaymentMethods(string customerId)
        {
            var apiResponse = await MakeRequest<Response<List<CustomerPaymentMethod>>>(Method.Get, $"/v1/customers/{customerId}/payment_methods");
            var result = apiResponse.Data;
            return result;
        }
        #endregion

        #region private methods
        private async Task<T> MakeRequest<T>(Method method, string urlPath, object body = null)
        {
            try
            {
                var request = PrepareRequest(method, urlPath, body);

                var response = await _client.ExecuteAsync<T>(request);

                if (response.ResponseStatus != ResponseStatus.Completed)
                {
                    throw new Exception(response.Data != null ? JsonSerializer.Serialize(response.Data) : response.Content);
                }
                return response.Data;
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error during API request: {ex}");
                throw;
            }
        }

        private string GenerateSign(string method, string urlPath, string salt, long timestamp, string body)
        {
            try
            {
                string bodyString = String.Empty;
                if (!String.IsNullOrWhiteSpace(body))
                {
                    bodyString = body == "{}" ? "" : body;
                }

                string toSign = method.ToLower() + urlPath + salt + timestamp + _accessKey + _secretKey + bodyString;

                UTF8Encoding encoding = new UTF8Encoding();
                byte[] secretKeyBytes = encoding.GetBytes(_secretKey);
                byte[] signatureBytes = encoding.GetBytes(toSign);
                string signature = String.Empty;
                using (HMACSHA256 hmac = new HMACSHA256(secretKeyBytes))
                {
                    byte[] signatureHash = hmac.ComputeHash(signatureBytes);
                    string signatureHex = String.Concat(Array.ConvertAll(signatureHash, x => x.ToString("x2")));
                    signature = Convert.ToBase64String(encoding.GetBytes(signatureHex));
                }

                return signature;
            }
            catch (Exception)
            {
                Console.WriteLine("Error generating signature");
                throw;
            }
        }

        private string GenerateRandomString(int size)
        {
            try
            {
                using (var rng = RandomNumberGenerator.Create())
                {
                    byte[] randomBytes = new byte[size];
                    rng.GetBytes(randomBytes);
                    return String.Concat(Array.ConvertAll(randomBytes, x => x.ToString("x2")));
                }
            }
            catch (Exception)
            {
                Console.WriteLine("Error generating salt");
                throw;
            }
        }

        private RestRequest PrepareRequest(Method method, string httpURLPath, object body = null)
        {
            try
            {
                var request = new RestRequest(httpURLPath);
                string httpBody = body is null ? "" : JsonSerializer.Serialize(body);
                string salt = GenerateRandomString(8);
                long timestamp = DateTimeOffset.Now.ToUnixTimeSeconds();
                string signature = GenerateSign(method.ToString(), httpURLPath, salt, timestamp, httpBody);

                request.Method = method;
                request.AddHeader("access_key", _accessKey);
                request.AddHeader("salt", salt);
                request.AddHeader("timestamp", timestamp.ToString());
                request.AddHeader("signature", signature);

                if (body != null)
                {
                    request.AddJsonBody(body);
                }
                return request;
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error generating request options: {ex}");
                throw;
            }
        }
        #endregion
    }
}
