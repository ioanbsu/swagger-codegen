/* 
 * Swagger Petstore
 *
 * This is a sample server Petstore server.  You can find out more about Swagger at [http://swagger.io](http://swagger.io) or on [irc.freenode.net, #swagger](http://swagger.io/irc/).  For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * OpenAPI spec version: 1.0.0
 * Contact: apiteam@swagger.io
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Pet : A pet for sale in the pet store

#[derive(Debug, Serialize, Deserialize)]
pub struct Pet {
  #[serde(rename = "id")] id: Option<i64>,
  #[serde(rename = "category")] category: Option<::models::Category>,
  #[serde(rename = "name")] name: String,
  #[serde(rename = "photoUrls")] photo_urls: Vec<String>,
  #[serde(rename = "tags")] tags: Option<Vec<::models::Tag>>,
  /// pet status in the store
  #[serde(rename = "status")] status: Option<String>
}

impl Pet {
  /// A pet for sale in the pet store
  pub fn new(name: String, photo_urls: Vec<String>) -> Pet {
    Pet {
      id: None,
      category: None,
      name: name,
      photo_urls: photo_urls,
      tags: None,
      status: None
    }
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> Pet {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> &i64 {
    &self.id
  }

  pub fn set_category(&mut self, category: ::models::Category) {
    self.category = Some(category);
  }

  pub fn with_category(mut self, category: ::models::Category) -> Pet {
    self.category = Some(category);
    self
  }

  pub fn category(&self) -> &::models::Category {
    &self.category
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> Pet {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn set_photo_urls(&mut self, photo_urls: Vec<String>) {
    self.photo_urls = photo_urls;
  }

  pub fn with_photo_urls(mut self, photo_urls: Vec<String>) -> Pet {
    self.photo_urls = photo_urls;
    self
  }

  pub fn photo_urls(&self) -> &Vec&lt;String&gt; {
    &self.photo_urls
  }

  pub fn set_tags(&mut self, tags: Vec<::models::Tag>) {
    self.tags = Some(tags);
  }

  pub fn with_tags(mut self, tags: Vec<::models::Tag>) -> Pet {
    self.tags = Some(tags);
    self
  }

  pub fn tags(&self) -> &Vec&lt;::models::Tag&gt; {
    &self.tags
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> Pet {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }

}



