use rocket::{get, response::content::RawJavaScript};

#[get("/idFetcher.js")]
pub fn id_fetcher_js() -> RawJavaScript<String> {
    let content = r#"
class IdFetcher {
  /** @type {string[]} */
  #ids = [];
  #url = 'api/cuid/10';

  constructor() {
    this.#fetchMore();
  }

  async #fetchMore() {
    try {
      const response = await fetch(this.#url);
      if (!response.ok) {
        throw new Error('Не получилось скачать айдишники');
      }
      const data = await response.json();
      this.#ids.push(...data.ids);
    } catch (err) {
      console.error(err);
    }
  }

  next() {
    const id = this.#ids.shift();
    if (this.#ids.length === 0) {
      this.#fetchMore();
    }
    return id;
  }
}

export const idFetcher = new IdFetcher();
    "#.to_string();
    RawJavaScript(content)
}


