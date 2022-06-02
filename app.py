from flask import Flask, render_template

app = Flask(__name__)


@app.route("/")
@app.route("/home")
def index():
    return render_template('index.html')


@app.route("/about_us")
def about_us():
    return render_template('about_us.html')


@app.route("/find")
def find():
    return render_template('find.html')


if __name__ == "__main__":
    app.run(debug=True)
