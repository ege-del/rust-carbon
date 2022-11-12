function add_note(text){
    $("#note-container").append(`
        <div class="card m-2">
            <div class="row p-1 ">
                <div class="text-target col-11">
                    ${text}
                </div>
                <div class="col-1 ">
                    <button class="h-100 delete-button p-0"></button>
                </div>
            <div>
          
        </div>
    `);
}
$( document ).ready(function() {
    add_note("notes asdas asd asd asdasd asd");
    add_note("notes asdas asd asd asdasd asd");
    add_note("notes asdas asd asd asdasd asd");
    add_note("notes asdas asd asd asdasd asd");
    add_note("notes asdas asd asd asdasd asd");
    add_note("notes asdas asd asd asdasd asd");
    $("#send_btn").click(function() {
        console.log(JSON.stringify({ text: $("#input_text").val() }));
        $.ajax({
            type: "POST",
            url: "/api/note/add",
            data: JSON.stringify({ text: $("#input_text").val() }),
            contentType: "application/json; charset=utf-8",
            dataType: "json",
            success: function(data){
                add_note($("#input_text").val());
            },
            error: function(errMsg) {
    
            }
        });
    });
});