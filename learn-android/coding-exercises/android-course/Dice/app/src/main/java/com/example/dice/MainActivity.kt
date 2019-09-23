package com.example.dice

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.Button
import android.widget.ImageView
import java.util.Random

class MainActivity : AppCompatActivity() {
    // save dice image reference must be set before calling it
    lateinit var diceImage: ImageView


    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        // Onclick listener for the button on mainActivity xml layout created
        findViewById<Button>(R.id.roll_button).setOnClickListener {
            rollDice()
        }
        // Save image reference so do not need to keep finding id.
        diceImage = findViewById(R.id.dice_image)
    }

    // Roll the dice function to generate a random number 1-6 and
    // convert to an image id to draw on the the activity
    private fun rollDice() {

        // create random number 1-6 and then check number and change to resource id
        val drawableResource = when (Random().nextInt(6) + 1) {
            1 -> R.drawable.dice_1
            2 -> R.drawable.dice_2
            3 -> R.drawable.dice_3
            4 -> R.drawable.dice_4
            5 -> R.drawable.dice_5
            else -> R.drawable.dice_6
        }
        // set resource id to image to display
        diceImage.setImageResource(drawableResource)

    }
}
