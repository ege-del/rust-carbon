

$( document ).ready(function() {
    $.ajax({
        type: "POST",
        url: "/api/note/add",
        data: JSON.stringify({ text: "FROM JQUERY" }),
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(data){

        },
        error: function(errMsg) {

        }
    });
});