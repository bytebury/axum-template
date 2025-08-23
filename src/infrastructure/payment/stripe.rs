use std::{collections, env, str::FromStr};

use stripe::{
    CheckoutSession, CheckoutSessionBillingAddressCollection, CheckoutSessionMode,
    CreateCheckoutSession, CreateCheckoutSessionAutomaticTax, CreateCheckoutSessionCustomerUpdate,
    CreateCheckoutSessionCustomerUpdateAddress, CreateCheckoutSessionLineItems, CustomerId,
};

use crate::models::user::User;

#[derive(Clone)]
pub struct Stripe {
    client: stripe::Client,
}

impl Stripe {
    pub fn new() -> Self {
        let secret_key = env::var("STRIPE_SECRET").expect("STRIPE_SECRET must be set");
        let client = stripe::Client::new(secret_key);
        Stripe { client }
    }

    /**
     * Create a Stripe customer if one does not already exist for the user.
     */
    async fn create_customer(&self, user: &User) -> Result<CustomerId, String> {
        if let Some(ref customer_id) = user.stripe_customer_id {
            return Ok(CustomerId::from_str(customer_id).unwrap());
        }

        let mut metadata = collections::HashMap::new();
        metadata.insert("user_id".to_string(), user.id.to_string());
        metadata.insert("email".to_string(), user.email.clone());
        metadata.insert("full_name".to_string(), user.full_name.clone());

        let customer = stripe::Customer::create(
            &self.client,
            stripe::CreateCustomer {
                name: Some(&user.full_name),
                email: Some(&user.email),
                metadata: Some(metadata),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| e.to_string())?;

        // TODO(Marcello): I need to update the stripe_customer_id in the database.

        Ok(customer.id)
    }

    pub async fn checkout(&self, user: &User, price_id: &str) -> Result<CheckoutSession, String> {
        let customer_id = self.create_customer(user).await?;

        let website_url = env::var("WEBSITE_URL").expect("WEBSITE_URL must be set");
        let success_url = format!("{website_url}/checkout/success");
        let cancel_url = format!("{website_url}/checkout/cancelled");

        let checkout_session = {
            let mut params = CreateCheckoutSession::new();
            params.cancel_url = Some(&cancel_url);
            params.success_url = Some(&success_url);
            params.customer = Some(CustomerId::from(customer_id));
            params.mode = Some(CheckoutSessionMode::Subscription);
            params.line_items = Some(vec![CreateCheckoutSessionLineItems {
                quantity: Some(1),
                price: Some(price_id.to_string()),
                ..Default::default()
            }]);
            params.automatic_tax = Some(CreateCheckoutSessionAutomaticTax {
                enabled: true,
                liability: None,
            });
            params.billing_address_collection =
                Some(CheckoutSessionBillingAddressCollection::Required);
            params.customer_update = Some(CreateCheckoutSessionCustomerUpdate {
                address: Some(CreateCheckoutSessionCustomerUpdateAddress::Auto),
                ..Default::default()
            });
            params.expand = &["line_items", "line_items.data.price.product"];

            CheckoutSession::create(&self.client, params).await.unwrap()
        };

        Ok(checkout_session)
    }
}
