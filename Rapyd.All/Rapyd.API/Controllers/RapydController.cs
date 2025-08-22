using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using Agrochain.API.Dto;
using Agrochain.API.Dto.Response;
using RestSharp;
using System.Security.Cryptography.Xml;
using Newtonsoft.Json.Linq;
using System.Text.Json;

namespace Agrochain.API.Controllers
{
    [Route("api/[controller]/[action]")]
    [ApiController]
    public class RapydController : ControllerBase
    {
        private readonly ILogger<RapydController> _logger;
        private readonly RapydClient _client;

        public RapydController(ILogger<RapydController> logger, RapydClient rapydClient)
        {
            _logger = logger;
            _client = rapydClient;
        }

        #region Card API
        [HttpPost("card/issue")]
        public async Task<ActionResult<CardIssueResponse>> IssueCard([FromBody] CardIssue cardDetails)
        {
            var apiResponse = await _client.IssueCard(cardDetails);
            return Ok(apiResponse);
        }

        [HttpPost("card/activate/{cardId}")]
        public async Task<ActionResult<CardIssueResponse>> ActivateIssuedCard([FromRoute] string cardId)
        {
            var apiResponse = await _client.ActivateIssuedCard(cardId);
            return Ok(apiResponse);
        }

        [HttpGet("card/{walletId}/all")]
        public async Task<ActionResult<ListOfIssuedCards>> ListOfIssueCard([FromRoute] string walletId, [FromQuery] int pagenumber = 0, [FromQuery] int pageSize = 0)
        {
            var apiResponse = await _client.ListOfIssueCard(walletId, pagenumber, pageSize);
            return Ok(apiResponse);
        }

        [HttpGet("card/{cardId}")]
        public async Task<ActionResult<CardIssueResponse>> GetIssuedCardById([FromRoute] string cardId)
        {
            var apiResponse = await _client.GetIssuedCardById(cardId);
            return Ok(apiResponse);
        }
        #endregion

        #region Wallet API
        [HttpPost("wallet/enable/{ewalletId}")]
        public async Task<ActionResult<bool>> EnableWallet([FromRoute] string ewalletId)
        {
            dynamic obj = new JObject();
            obj.ewallet = ewalletId;

            var apiResponse = await _client.EnableWallet(obj);
            return Ok(apiResponse);
        }

        [HttpGet("wallet/all")]
        public async Task<ActionResult<ListOfWallet>> GetListOfWallet(string type, Guid referenceId, int pagenumber = 0, int pageSize = 0)
        {
            var apiResponse = await _client.GetListOfWallet(type, referenceId, pagenumber, pageSize);
            return Ok(apiResponse); ;
        }

        [HttpPost("wallet/new")]
        public async Task<ActionResult<CreateWalletResponse?>> CreateNewWallet([FromBody] CreateWallet walletDetails)
        {
            var address = new
            {
                name = "John Doe",
                line_1 = "123 Main Street",
                line_2 = "",
                line_3 = "",
                city = "Anytown",
                state = "NY",
                country = "US",
                zip = "12345",
                metadata = new { },
                canton = "",
                district = ""
            };

            var metadata = new
            {
                merchant_defined = true
            };

            var contact = new
            {
                email = "johndoe@rapyd.net",
                first_name = "John",
                last_name = "Doe",
                mothers_name = "Jane Smith",
                contact_type = "personal",
                address,
                identification_type = "PA",
                identification_number = "1234567890",
                date_of_birth = "11/22/2000",
                country = "US",
                metadata,
            };

            var requestObj = new
            {
                first_name = "John",
                last_name = "Doe",
                email = "",
                ewallet_reference_id = "John-Doe-02152020",
                metadata,
                phone_number = "",
                type = "person",
                contact,
                country = "US",
            };

            string request = JsonSerializer.Serialize(requestObj);

            var apiResponse = await _client.CreateNewWallet(request);
            return Ok(apiResponse); ;
        }
        #endregion

        #region customer APIs
        [HttpGet("customers")]
        public async Task<ActionResult<IList<Customer>>> GetCustomers()
        {
            var customers = await _client.GetCustomers();
            return Ok(customers);
        }

        [HttpPost("customers")]
        public async Task<ActionResult<Customer>> CreateCustomer([FromBody] CreateCustomerBody body)
        {
            var customer = await _client.CreateCustomer(body);
            return Ok(customer);
        }
        #endregion

        #region payment APIs
        [HttpGet("customers/{customerId}/paymentMethods")]
        public async Task<ActionResult<List<CustomerPaymentMethod>>> GetCustomerPaymentMethods([FromRoute] string customerId)
        {
            var customer = await _client.GetCustomersPaymentMethods(customerId);
            return Ok(customer);
        }

        //[HttpGet("paymentMethods")]
        //public async Task<ActionResult<IList<PaymentMethod>>> GetPaymentMethods([FromQuery] string country)
        //{
        //    var methods = await _client.GetPaymentMethods(country);
        //    return Ok(methods);
        //}

        //[HttpGet("paymentMethodRequiredFields/{type}")]
        //public async Task<ActionResult<PaymentMethodRequiredFields>> GetPaymentMethodRequiredFields([FromRoute] string type)
        //{
        //    var methods = await _client.GetPaymentMethodRequiredFields(type);
        //    return Ok(methods);
        //}

        [HttpPost("checkout")]
        public async Task<ActionResult<Checkout>> CreateCheckout([FromBody] CreateCheckoutBody body)
        {
            var checkout = await _client.CreatePaymentCheckout(body);
            return Ok(checkout);
        }

        [HttpPost("payment")]
        public async Task<ActionResult<CardPaymentResponse>> CreatePayment([FromBody] CardPayment body)
        {
            var payment = await _client.CreatePayment(body);
            return Ok(payment);
        }

        [HttpDelete("payment/{paymentId}")]
        public async Task<ActionResult<Payment>> CancelPayment([FromRoute] string paymentId)
        {
            var payment = await _client.CancelPayment(paymentId);
            return Ok(payment);
        }
        #endregion
    }
}
