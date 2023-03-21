
require('dotenv').config()

const { readFileSync, writeFileSync } = require('jsonfile')
const { Webhook, MessageBuilder } = require("discord-webhook-node")
const rss = new (require('rss-parser'))()

const hook = new Webhook(process.env.WEBHOOK)
const library = {
    ambu: 'https://files.thijmenheuvelink.nl/api/public/dl/ginQaA4N/ambu.png',
    brand: 'https://files.thijmenheuvelink.nl/api/public/dl/ginQaA4N/brand.png',
    ongeval: 'https://files.thijmenheuvelink.nl/api/public/dl/ginQaA4N/ongeval.png',
    politie: 'https://files.thijmenheuvelink.nl/api/public/dl/ginQaA4N/politie.png',
    trauma: 'https://files.thijmenheuvelink.nl/api/public/dl/ginQaA4N/trauma.png',
}

function assetMatch(content) {
    const query = content.toLowerCase()

    if (query.includes('ambu')) return library['ambu']
    if (query.includes('brand')) return library['brand']
    if (query.includes('politie')) return library['politie']
    if (query.includes('trauma')) return library['trauma']
    return library['ongeval']
}

function poll() {
    rss.parseURL(process.env.RSS)
        .then(({ items }) => {
            let cache = readFileSync('./cache.json')

            items.reverse().forEach(item => {
                if (cache.find(({ guid }) => guid == item.guid)) return

                cache.push(item)

                console.log('Sent:', item)

                const embed = new MessageBuilder()
                    .setColor('#e36549')
                    .setURL(item.link)
                    .setTitle(item.content)
                    .setTimestamp(item.isoDate)
                    .setImage(assetMatch(item.content))

                hook.send(embed)
            })

            writeFileSync('./cache.json', cache)
            console.log('Done, next poll in 15m.\n')
        })
}

poll()
setInterval(() => poll(), 15 * 60 * 1000)

process.on('uncaughtException', err => console.error(err))
