  window.addEventListener("DOMContentLoaded", function () {
    // topnav
    var topButton = document.getElementById("topnavButton");
    var topnavList = document.getElementById("topnavList");
    if (topButton && topnavList) {
      topButton.addEventListener("click", function (e) {
        topnavList
          .classList
          .toggle("hidden");
        e.stopPropagation();
      });
      document.addEventListener("click", function (e) {
        // if the clicked element isn't the list, close the list
        if (!topnavList.classList.contains("hidden") && !e.target.closest("#topnavList")) {
          topButton.click();
        }
      });

      // set active <a> element
      var url = window
        .location
        .href
        .toLowerCase();
      var aTags = topnavList.getElementsByTagName('a');
      for (var i = 0; i < aTags.length; i++) {
        var lowerhref = aTags[i]
          .href
          .toLowerCase();
        if (url.startsWith(lowerhref)) {
          aTags[i]
            .classList
            .add('active');
          break;
        }
      }
    }
  });

