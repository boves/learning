const Nightmare = require('nightmare')
const nightmare = Nightmare({ show: true })
const selector = '.wob_t'
const fn = (selector) => {
  return document.querySelector(selector).innerText
}

nightmare
  .goto('https://google.com/search?q=weather+ireland')
  .wait(selector)
  .evaluate(fn, selector)
  .end()
  .catch(error => {
    console.error('Search failed:', error)
  })