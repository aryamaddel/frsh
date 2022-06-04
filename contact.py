import email
from flask_wtf import FlaskForm
from wtforms import StringField, SubmitField
from wtforms.validators import DataRequired, Length, Email


class contact_from(FlaskForm):
    email = StringField('Email address', validators=[DataRequired(), Email()])
    message = StringField('Message for us')
    submit = SubmitField('submit')
