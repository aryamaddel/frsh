import email
from flask_wtf import FlaskForm
from wtforms import StringField, SubmitField
from wtforms.validators import DataRequired, Length, Email


class contact_from(FlaskForm):
    email = StringField('email', validators=[DataRequired(), Email()])
    submit = SubmitField('submit')
