use serde::Deserialize;

#[derive(Deserialize)]
#[allow(non_snake_case, dead_code)]
pub struct Volumens {
    kind: String,
    totalItems: usize,
    items: Vec<Item>,
}
#[derive(Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct Item {
    kind: String,
    id: String,
    etag: String,
    selfLink: String,
    volumeInfo: VolumeInfo,
    saleInfo: SaleInfo,
    accessInfo: AccessInfo,
    searchInfo: SearchInfo,
}
#[derive(Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct AccessInfo {
    country: String,
    viewability: String,
    embeddable: bool,
    publicDomain: bool,
    textToSpeechPermission: String,
    epub: IsAvailable,
    pdf: IsAvailable,
    webReaderLink: String,
    accessViewStatus: String,
    quoteSharingAllowed: bool,
}
#[derive(Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct IsAvailable {
    isAvailable: bool,
    acsTokenLink: Option<String>,
}
#[derive(Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct SaleInfo {
    country: String,
    saleability: String,
    isEbook: bool,
    listPrice: Option<Prices>,
    retailPrice: Option<Prices>,
    buyLink: Option<String>,
    offers: Option<Vec<Offers>>,
}
#[derive(Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct Prices {
    amount: usize,
    currencyCode: String,
}
#[derive(Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct SearchInfo {
    textSnippet: String,
}
#[derive(Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct Offers {
    finskyOfferType: usize,
    listPrice: PricesOffer,
    retailPrice: PricesOffer,
}
#[derive(Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct PricesOffer {
    amountInMicros: usize,
    currencyCode: String,
}
#[derive(Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct VolumeInfo {
    title: String,
    authors: Vec<String>,
    publishedDate: String,
    description: String,
    publisher: Option<String>,
    panelizationSummary: Option<Panelization>,
    industryIdentifiers: Vec<IndustryIdentifier>,
    readingModes: ReadingModes,
    pageCount: usize,
    printType: String,
    categories: Vec<String>,
    averageRating: Option<usize>,
    ratingsCount: Option<usize>,
    maturityRating: String,
    allowAnonLogging: bool,
    contentVersion: String,
    imageLinks: Option<ImageLinks>,
    language: String,
    previewLink: String,
    infoLink: String,
    canonicalVolumeLink: String,
}
#[derive(Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct Panelization {
    containsEpubBubbles: bool,
    containsImageBubbles: bool,
}
#[derive(Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct ImageLinks {
    smallThumbnail: String,
    thumbnail: String,
}
#[derive(Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct IndustryIdentifier {
    r#type: String,
    identifier: String,
}
#[derive(Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct ReadingModes {
    text: bool,
    image: bool,
}
