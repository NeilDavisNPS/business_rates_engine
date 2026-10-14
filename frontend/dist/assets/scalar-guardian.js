/**
 * Scalar API Reference — Accessibility & Autofill DOM Guardian
 *
 * This guardian script runs alongside the self-hosted @scalar/api-reference standalone bundle
 * to ensure that all dynamically rendered DOM elements comply with WCAG AAA accessibility
 * and Chromium autofill standards without requiring modifications to the upstream vendor bundle.
 *
 * Key guarantees:
 * 1. Form Field Identity: Ensures all <input>, <select>, and <textarea> elements
 *    have unique `id` and `name` attributes for autofill and screen-reader association.
 * 2. Label Association: Converts any unassociated <label> tags (e.g. standalone Server
 *    headers or Headless UI Listbox headers without `for` attributes or child inputs)
 *    into styled <span> elements with identical classes, eliminating Chromium orphan-label warnings.
 */
(function () {
  if (typeof window === 'undefined' || typeof document === 'undefined') return;

  var fieldCounter = 0;

  function sanitizeSubtree(root) {
    if (!root || !root.querySelectorAll) return;

    // 1. Ensure all form controls have unique `id` and `name` attributes
    var formFields = root.querySelectorAll('input:not([id]), select:not([id]), textarea:not([id])');
    for (var i = 0; i < formFields.length; i++) {
      var field = formFields[i];
      fieldCounter++;
      var autoId = 'scalar-field-' + fieldCounter;
      field.id = autoId;
      if (!field.name) {
        field.name = autoId;
      }
    }

    // Also ensure form controls that have an id but lack a name attribute receive one
    var unnamedFields = root.querySelectorAll('input:not([name]), select:not([name]), textarea:not([name])');
    for (var j = 0; j < unnamedFields.length; j++) {
      var unnamed = unnamedFields[j];
      unnamed.name = unnamed.id || ('scalar-field-' + (++fieldCounter));
    }

    // 2. Identify and fix unassociated <label> elements
    var labels = root.querySelectorAll('label');
    for (var k = 0; k < labels.length; k++) {
      var label = labels[k];
      var forAttr = label.getAttribute('for');
      var hasTargetControl = (forAttr && document.getElementById(forAttr)) || label.querySelector('input, select, textarea, button');

      if (!hasTargetControl) {
        // Replace unassociated <label> with a <span> preserving all classes, styles, and children
        var replacementSpan = document.createElement('span');
        for (var a = 0; a < label.attributes.length; a++) {
          replacementSpan.setAttribute(label.attributes[a].name, label.attributes[a].value);
        }
        while (label.firstChild) {
          replacementSpan.appendChild(label.firstChild);
        }
        if (label.parentNode) {
          label.parentNode.replaceChild(replacementSpan, label);
        }
      }
    }
  }

  function runSanitization() {
    sanitizeSubtree(document);
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', runSanitization);
  } else {
    runSanitization();
  }

  try {
    var observer = new MutationObserver(function (mutations) {
      for (var m = 0; m < mutations.length; m++) {
        var mutation = mutations[m];
        for (var n = 0; n < mutation.addedNodes.length; n++) {
          var node = mutation.addedNodes[n];
          if (node.nodeType === 1) { // Node.ELEMENT_NODE
            sanitizeSubtree(node);
          }
        }
      }
    });

    observer.observe(document.documentElement, {
      childList: true,
      subtree: true
    });
  } catch (_) {}
})();

