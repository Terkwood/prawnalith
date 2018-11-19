// Thanks to google for making this available under Apache 2 license.

/**
 * Common methods for both the main app page and standalone widget.
 */

/**
 * @return {string} The reCAPTCHA rendering mode from the configuration.
 */
function getRecaptchaMode() {
  // Quick way of checking query params in the fragment. If we add more config
  // we might want to actually parse the fragment as a query string.
  return location.hash.indexOf('recaptcha=invisible') !== -1 ?
      'invisible' : 'normal';
}
