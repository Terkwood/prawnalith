/**
 * FirebaseUI initialization to be used in a Single Page application context.
 */

 

/**
 * @return {!Object} The FirebaseUI config.
 */
function getUiConfig() {
    return {
        'callbacks': {
        // Called when the user has been successfully signed in.
        'signInSuccessWithAuthResult': function(authResult, redirectUrl) {
            if (authResult.user) {
                handleSignedInUser(authResult.user);
            }
            if (authResult.additionalUserInfo) {
            window.document.getElementById('is-new-user').textContent =
                authResult.additionalUserInfo.isNewUser ?
                'New User' : 'Existing User';
            }
            // Do not redirect.
            return false;
        }
        },
        // Opens IDP Providers sign-in flow in a popup.
        'signInFlow': 'popup',
        'signInOptions': [
        // TODO(developer): Remove the providers you don't need for your app.
        {
            provider: firebase.auth.GoogleAuthProvider.PROVIDER_ID,
            // Required to enable this provider in One-Tap Sign-up.
            authMethod: 'https://accounts.google.com',
            // Required to enable ID token credentials for this provider.
            clientId: CLIENT_ID
        }
        ],
        // Terms of service url.
        'tosUrl': 'https://www.google.com',
        // Privacy policy url.
        'privacyPolicyUrl': 'https://www.google.com',
        'credentialHelper': CLIENT_ID && CLIENT_ID != 'YOUR_OAUTH_CLIENT_ID' ?
            firebaseui.auth.CredentialHelper.GOOGLE_YOLO :
            firebaseui.auth.CredentialHelper.ACCOUNT_CHOOSER_COM
    };
}


/**
 * @return {string} The URL of the FirebaseUI standalone widget.
 */
function getWidgetUrl() {
    return '/widget#recaptcha=' + getRecaptchaMode();
}


/**
 * Redirects to the FirebaseUI widget.
 */
var signInWithRedirect = function() {
    window.location.assign(getWidgetUrl());
};


/**
 * Open a popup with the FirebaseUI widget.
 */
var signInWithPopup = function() {
    window.open(getWidgetUrl(), 'Sign In', 'width=985,height=735');
};


/**
 * Displays the UI for a signed in user.
 * @param {!firebase.User} user
 */
var handleSignedInUser = function(user) {
    window.document.getElementById('user-signed-in').style.display = 'block';
    window.document.getElementById('user-signed-out').style.display = 'none';
    window.document.getElementById('name').textContent = user.displayName;
    window.document.getElementById('email').textContent = user.email;
    window.document.getElementById('phone').textContent = user.phoneNumber;
    if (user.photoURL){
        var photoURL = user.photoURL;
        // Append size to the photo URL for Google hosted images to avoid requesting
        // the image with its original resolution (using more bandwidth than needed)
        // when it is going to be presented in smaller size.
        if ((photoURL.indexOf('googleusercontent.com') != -1) ||
            (photoURL.indexOf('ggpht.com') != -1)) {
        photoURL = photoURL + '?sz=' +
            window.document.getElementById('photo').clientHeight;
        }
        window.document.getElementById('photo').src = photoURL;
        window.document.getElementById('photo').style.display = 'block';
    } else {
        window.document.getElementById('photo').style.display = 'none';
    }
};


/**
 * Displays the UI for a signed out user.
 */
var handleSignedOutUser = function() {
    window.document.getElementById('user-signed-in').style.display = 'none';
    window.document.getElementById('user-signed-out').style.display = 'block';
    ui.start('#firebaseui-container', getUiConfig());
    };

    // Listen to change in auth state so it displays the correct UI for when
    // the user is signed in or not.
    firebase.auth().onAuthStateChanged(function(user) {
    window.document.getElementById('loading').style.display = 'none';
    window.document.getElementById('loaded').style.display = 'block';
    user ? handleSignedInUser(user) : handleSignedOutUser();
});

/**
 * Deletes the user's account.
 */
var deleteAccount = function() {
    firebase.auth().currentUser.delete().catch(function(error) {
        if (error.code == 'auth/requires-recent-login') {
        // The user's credential is too old. She needs to sign in again.
        firebase.auth().signOut().then(function() {
            // The timeout allows the message to be displayed after the UI has
            // changed to the signed out state.
            setTimeout(function() {
            alert('Please sign in again to delete your account.');
            }, 1);
        });
        }
    });
};


/**
 * Initializes the app.
 */
var initApp = function() {
    window.document.getElementById('sign-out').addEventListener('click', function() {
        firebase.auth().signOut();
    });
    window.document.getElementById('delete-account').addEventListener(
        'click', function() {
            deleteAccount();
        });
};

// Initialize the FirebaseUI Widget using Firebase.
var ui = new firebaseui.auth.AuthUI(firebase.auth());
// Disable auto-sign in.
ui.disableAutoSignIn();

window.addEventListener('load', initApp);

/**
 * @return {string} The reCAPTCHA rendering mode from the configuration.
 */
function getRecaptchaMode() {
    // Quick way of checking query params in the fragment. If we add more config
    // we might want to actually parse the fragment as a query string.
    return location.hash.indexOf('recaptcha=invisible') !== -1 ?
        'invisible' : 'normal';
}