import { parse } from 'node-html-parser'

const BLOG_URL =
  process.env.BLOG_WEBFLOW_URL || 'https://blog-bd2f0f.webflow.io'

// cleanup wordpress page and parse html
export const parseHtml = async (body: string) => {
  const htmlRoot = parse(body)

  const siteNavigationAnchor = htmlRoot.querySelector('#site-navigation a')
  const footer = htmlRoot.querySelector('.footer-wrap')
  const navMenu = htmlRoot.querySelector('.menu-area')
  const statsScript = htmlRoot.querySelector(
    `script[src^="https://stats.wp.com"]`
  )
  const blurScript = htmlRoot.querySelector(
    `script[src^="https://s0.wp.com/wp-content/js/bilmur.min.js"]`
  )
  const h1Tags = htmlRoot.querySelectorAll(`h1`)
  const likesIframe = htmlRoot.querySelector(`#likes-master`)

  const shareSection = htmlRoot.querySelectorAll(`.sharedaddy`)
  const followHeading = htmlRoot.querySelector('#follow-our-blog')

  if (followHeading) {
    const followHeadingSubtitle = followHeading?.nextElementSibling
    // remove jetpack custom follow sections trail
    followHeadingSubtitle?.nextElementSibling?.remove()
    followHeadingSubtitle?.remove()
    followHeading?.remove()
  }

  // wordpress theme scripts - disable for app analytics & bluring
  statsScript?.remove()
  blurScript?.remove()
  likesIframe?.remove()

  h1Tags?.forEach((h1, index) => {
    if (h1Tags.length > 1) {
      // replace the navbar h1 to span
      if (index === 0) {
        let clone = h1
        clone.tagName = 'span'
        h1.replaceWith(clone)
        // let the second H1 stick (issue with wp theme)
      } else if (index > 1) {
        let clone = h1
        clone.tagName = 'h2'

        h1.replaceWith(clone)
      }
    }
  })

  const blogAnchors = htmlRoot.querySelectorAll(
    `a[href^="${BLOG_URL}"],a[href^="/"]`
  )
  // manipulate links that are blog pages relativeness
  blogAnchors.forEach((link) => {
    const url = link.getAttribute('href') || ''
    if (url) {
      // convert all to relative
      let urlBase = url.replace(BLOG_URL, '')

      if (process.env.NODE_ENV === 'development' && !url.startsWith('/blog')) {
        urlBase = `/blog${url}`
      }
      link.setAttribute('href', urlBase)
    }
  })

  shareSection?.forEach((tag) => {
    tag.remove()
  })

  footer?.remove()
  siteNavigationAnchor?.remove()
  navMenu?.remove()

  htmlRoot.insertAdjacentHTML(
    'beforeend',
    `<style type="text/css">
            .light-background article > .entry-wrapper > p {
              max-width: none;
            }
            .light-background #content, #comments {
              padding-top: 20px;
              padding-bottom: 20px;
              overflow: hidden;
            }
            .light-background {
              background-color: #fff;
              font-family: system-ui;
            }
            .w-dyn-list,.grid-2-columns.hero-top {
              opacity: 1 !important;
            }
            .blog-card-bg-image {
              z-index: 1 !important;
            }
        </style>`
  )

  htmlRoot.removeWhitespace()

  return htmlRoot
}
