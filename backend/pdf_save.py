import fitz
import io
import os
from PIL import Image

PATH = os.path.dirname(os.path.realpath(__file__))

def save_imgs_from_pdf(path, hexdigest) -> int:
    uploaded_pdf = fitz.open(path)
    picture_count = 0

    for page in uploaded_pdf:
        for img in page.get_images():
            # get the XREF of the image
            xref = img[0]
            
            # extract the image bytes
            base_image = uploaded_pdf.extract_image(xref)
            image_bytes = base_image["image"]

            # get the image extension
            image_ext = base_image["ext"]

            img = Image.open(io.BytesIO(image_bytes))

            picture_path = f"{hexdigest}_{picture_count}.{image_ext}"

            img.save(f"{PATH}/static/images/{picture_path}")

            picture_count+=1
    return picture_count