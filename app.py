from turtle import title
from flask import Flask, render_template
from contact import contact_from

app = Flask(__name__)

app.config['SECRET_KEY'] = '0dd408513c47689af536fe9c3a6c7e90'


@app.route("/")
@app.route("/home")
def index():
    contact_form = contact_from()
    return render_template('index.html', title="home", form=contact_form)


@app.route("/about_us")
def about_us():
    return render_template('about_us.html')


@app.route("/find")
def find():
    return render_template('find.html')


if __name__ == "__main__":
    app.run(debug=True)
